//! SigmaOS-agile: Agile methodology model.
//!
//! Real implementation: sprints, stories, validation.
//! Replaces the 31-line stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sprint {
    pub id: String,
    pub name: String,
    pub duration: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Story {
    pub id: String,
    pub sprint_id: String,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgileError {
    EmptySprintName,
    DuplicateSprint(String),
}

impl fmt::Display for AgileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySprintName => write!(f, "sprint name must not be empty"),
            Self::DuplicateSprint(id) => write!(f, "duplicate sprint {:?}", id),
        }
    }
}

impl std::error::Error for AgileError {}

pub struct Agile {
    pub name: String,
    sprints: BTreeMap<String, Sprint>,
    stories: BTreeMap<String, Story>,
}

impl Agile {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), sprints: BTreeMap::new(), stories: BTreeMap::new() }
    }

    pub fn add_sprint(&mut self, s: Sprint) -> Result<(), AgileError> {
        if s.name.trim().is_empty() { return Err(AgileError::EmptySprintName); }
        let id = s.id.clone();
        if self.sprints.contains_key(&id) {
            return Err(AgileError::DuplicateSprint(id.clone()));
        }
        self.sprints.insert(id, s);
        Ok(())
    }

    pub fn sprint_count(&self) -> usize { self.sprints.len() }

    pub fn get_sprint(&self, id: &str) -> Option<&Sprint> { self.sprints.get(id) }

    pub fn add_story(&mut self, s: Story) {
        self.stories.insert(s.id.clone(), s);
    }

    pub fn get_story(&self, id: &str) -> Option<&Story> { self.stories.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let a = Agile::new("Agile");
        assert_eq!(a.name, "Agile");
        assert_eq!(a.sprint_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut a = Agile::new("Agile");
        let sprint = Sprint { id: "s1".into(), name: "Sprint 1".into(), duration: "2 weeks".into() };
        a.add_sprint(sprint).unwrap();
        assert_eq!(a.sprint_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut a = Agile::new("Agile");
        let sprint = Sprint { id: "x".into(), name: "".into(), duration: "x".into() };
        assert!(a.add_sprint(sprint).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut a = Agile::new("Agile");
        let sprint = Sprint { id: "x".into(), name: "X".into(), duration: "x".into() };
        a.add_sprint(sprint).unwrap();
        let sprint2 = Sprint { id: "x".into(), name: "X".into(), duration: "x".into() };
        assert!(a.add_sprint(sprint2).is_err());
    }
}
