use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::From;
use std::iter::FromIterator;

/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct InputCellID(usize);

/// `ComputeCellID` is a unique identifier for a compute cell.
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct ComputeCellID(usize);

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct CallbackID(usize);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

impl From<usize> for InputCellID {
    fn from(val: usize) -> Self {
        InputCellID(val)
    }
}

impl From<usize> for ComputeCellID {
    fn from(val: usize) -> Self {
        ComputeCellID(val)
    }
}

impl Into<CellID> for InputCellID {
    fn into(self) -> CellID {
        CellID::Input(self)
    }
}

impl Into<CellID> for ComputeCellID {
    fn into(self) -> CellID {
        CellID::Compute(self)
    }
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct InputCell<T> {
    value: T,
    dest_ids: Vec<ComputeCellID>, // Should be sorted
}

struct ComputeCell<'a, 'b, T> {
    value: T,
    dest_ids: Vec<ComputeCellID>, // Should be sorted
    src_ids: Vec<CellID>,         // Should NOT be sorted
    comp_fn: Box<'a + Fn(&[T]) -> T>,
    callbacks: HashMap<CallbackID, Box<'b + FnMut(T) -> ()>>,
    next_callback_id: usize,
}

pub struct Reactor<'a, 'b, T> {
    input_cells: Vec<InputCell<T>>,
    compute_cells: Vec<ComputeCell<'a, 'b, T>>,
}

fn partition_ids<I>(iter: I) -> (Vec<InputCellID>, Vec<ComputeCellID>)
where
    I: IntoIterator<Item = CellID>,
{
    let mut inpt_ids: Vec<InputCellID> = vec![];
    let mut comp_ids: Vec<ComputeCellID> = vec![];
    for id in iter {
        match id {
            CellID::Input(inpt_id) => {
                inpt_ids.push(inpt_id);
            }
            CellID::Compute(comp_id) => {
                comp_ids.push(comp_id);
            }
        };
    }
    (inpt_ids, comp_ids)
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, 'b, T: Copy + PartialEq> Reactor<'a, 'b, T> {
    pub fn new() -> Self {
        Reactor {
            input_cells: Vec::new(),
            compute_cells: Vec::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellID {
        let input_cell = InputCell {
            value: initial,
            dest_ids: vec![],
        };
        self.input_cells.push(input_cell);
        (self.input_cells.len() - 1).into()
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: 'a + Fn(&[T]) -> T>(
        &mut self,
        dependencies: &[CellID],
        compute_func: F,
    ) -> Result<ComputeCellID, CellID> {
        let (inpt_ids, comp_ids) = partition_ids(dependencies.to_vec());
        for &id in inpt_ids.iter() {
            if id.0 >= self.input_cells.len() {
                return Err(id.into());
            }
        }
        for &id in comp_ids.iter() {
            if id.0 >= self.compute_cells.len() {
                return Err(id.into());
            }
        }
        let comp_id = self.compute_cells.len().into();
        for id in inpt_ids {
            self.input_cells[id.0].dest_ids.push(comp_id);
        }
        for id in comp_ids {
            self.compute_cells[id.0].dest_ids.push(comp_id);
        }
        let value = compute_func(self.gather_input_vals(dependencies).as_slice());

        let comp_cell = ComputeCell {
            value,
            dest_ids: vec![],
            src_ids: dependencies.to_vec(),
            comp_fn: Box::new(compute_func),
            callbacks: HashMap::new(),
            next_callback_id: 0,
        };
        self.compute_cells.push(comp_cell);
        Ok(comp_id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    pub fn value(&self, cid: CellID) -> Option<T> {
        match cid {
            CellID::Input(id) => self.input_cells.get(id.0).map(|c| c.value),
            CellID::Compute(id) => self.compute_cells.get(id.0).map(|c| c.value),
        }
    }

    // For a slice of CellID's, return a vector with their current values.
    fn gather_input_vals(&self, src_ids: &[CellID]) -> Vec<T> {
        let mut input_vals: Vec<T> = Vec::with_capacity(src_ids.len());
        for cid in src_ids.iter() {
            match cid {
                CellID::Input(id) => {
                    input_vals.push(self.input_cells[id.0].value);
                }
                CellID::Compute(id) => {
                    input_vals.push(self.compute_cells[id.0].value);
                }
            }
        }
        input_vals
    }

    // Compute the value of a compute cell. Returns true if the value changed; false otherwise
    fn update_compute_cell(&mut self, id: ComputeCellID) -> bool {
        let src_ids = &self.compute_cells[id.0].src_ids;
        let input_vals: Vec<T> = self.gather_input_vals(src_ids);
        let new_val = (*self.compute_cells[id.0].comp_fn)(input_vals.as_slice());
        let changed = self.compute_cells[id.0].value != new_val;
        self.compute_cells[id.0].value = new_val;
        changed
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    pub fn set_value(&mut self, id: InputCellID, new_value: T) -> bool {
        if id.0 >= self.input_cells.len() {
            return false;
        }
        if self.input_cells[id.0].value == new_value {
            return true;
        }
        self.input_cells[id.0].value = new_value;

        // The dependency tree forms a directed, acyclic graph (DAG),
        // where a the following invariants (SHOULD!) hold:
        // 1) A input cell cannot depend on another cell
        //    (thus only a single input cell has its value changed during this method call)
        // 2) A compute cell can only depend on other compute cells with lower ID, since
        //    a) cells cannot be removed
        //    b) upon creation, a compute cell will get a higher ID than any
        //       other existing compute cell
        //    c) upon creation, a cell cannot depend upon cells created in the future

        let mut processed: HashMap<ComputeCellID, bool> = HashMap::new();
        let mut unprocessed_set: HashSet<ComputeCellID> =
            HashSet::from_iter(self.input_cells[id.0].dest_ids.to_owned().into_iter());
        let mut unprocessed_heap: BinaryHeap<Reverse<ComputeCellID>> = BinaryHeap::from_iter(
            self.input_cells[id.0]
                .dest_ids
                .iter()
                .map(|&cid| Reverse(cid)),
        );

        while let Some(Reverse(comp_id)) = unprocessed_heap.pop() {
            let value_changed = self.update_compute_cell(comp_id);
            processed.insert(comp_id, value_changed);
            for dest_comp_id in self.compute_cells[comp_id.0].dest_ids.iter() {
                if !processed.contains_key(dest_comp_id) && !unprocessed_set.contains(dest_comp_id)
                {
                    unprocessed_heap.push(Reverse(*dest_comp_id));
                    unprocessed_set.insert(*dest_comp_id);
                }
            }
        }

        for (comp_id, value_changed) in processed.drain() {
            if value_changed {
                let comp_cell = &mut self.compute_cells[comp_id.0];
                for callback in comp_cell.callbacks.values_mut() {
                    callback(comp_cell.value);
                }
            }
        }
        true
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: 'b + FnMut(T) -> ()>(
        &mut self,
        id: ComputeCellID,
        callback: F,
    ) -> Option<CallbackID> {
        if let Some(comp_cell) = self.compute_cells.get_mut(id.0) {
            let callback_id = CallbackID(comp_cell.next_callback_id);
            comp_cell.callbacks.insert(callback_id, Box::new(callback));
            // comp_cell.callbacks.insert(callback_id, callback);
            comp_cell.next_callback_id += 1;
            Some(callback_id)
        } else {
            None
        }
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        self.compute_cells
            .get_mut(cell.0)
            .ok_or(RemoveCallbackError::NonexistentCell)
            .and_then(|comp_cell| {
                comp_cell
                    .callbacks
                    .remove(&callback)
                    .ok_or(RemoveCallbackError::NonexistentCallback)
            })
            .map(|_| ())
    }
}
