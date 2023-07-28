use crate::models::{
    agent_basic::basic_agent::AgentState,
    agents::agent_triaits::{FactSheet, SpecialFunctioins},
};

pub struct ManaingAgent {
    attribute: AgentState,
    factsheet: FactSheet,
    agents: Vec<Box<dyn SpecialFunctioins>>,
}
