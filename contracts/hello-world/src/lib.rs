#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short, Address};

// Project status enum
#[contracttype]
#[derive(Clone)]
pub enum ProjectStatus {
    Pending,
    InProgress,
    Completed,
    Disputed
}

// Design Project structure
#[contracttype]
#[derive(Clone)]
pub struct DesignProject {
    pub project_id: u64,
    pub client: Address,
    pub designer: Address,
    pub title: String,
    pub description: String,
    pub price: u64,
    pub created_at: u64,
    pub completed_at: u64,
    pub status: ProjectStatus,
}

// For tracking total projects created
const PROJECT_COUNT: Symbol = symbol_short!("PRJ_COUNT");

// Mapping project_id to its DesignProject struct
#[contracttype]
pub enum ProjectBook {
    Project(u64)
}

#[contract]
pub struct FreelanceDesignPlatform;

#[contractimpl]
impl FreelanceDesignPlatform {
    // Create a new design project
    pub fn create_project(
        env: Env, 
        client: Address, 
        designer: Address, 
        title: String, 
        description: String, 
        price: u64
    ) -> u64 {
        // Get current project count and increment it
        let mut project_count: u64 = env.storage().instance().get(&PROJECT_COUNT).unwrap_or(0);
        project_count += 1;
        
        // Get current timestamp
        let timestamp = env.ledger().timestamp();
        
        // Create new project
        let project = DesignProject {
            project_id: project_count,
            client: client,
            designer: designer,
            title: title,
            description: description,
            price: price,
            created_at: timestamp,
            completed_at: 0,
            status: ProjectStatus::Pending,
        };
        
        // Store the project
        env.storage().instance().set(&ProjectBook::Project(project_count), &project);
        
        // Update project count
        env.storage().instance().set(&PROJECT_COUNT, &project_count);
        
        // Extend storage lifetime
        env.storage().instance().extend_ttl(5000, 5000);
        
        log!(&env, "Project created with ID: {}", project_count);
        
        return project_count;
    }
    
    // Update project status (can be called by designer or client depending on the status change)
    pub fn update_project_status(env: Env, project_id: u64, new_status: ProjectStatus) -> ProjectStatus {
        let key = ProjectBook::Project(project_id);
        
        // Retrieve the project
        let mut project: DesignProject = env.storage().instance().get(&key)
            .expect("Project not found");
            
        // Update status
        project.status = new_status.clone();
        
        // If status is set to Completed, update completion timestamp
        if matches!(new_status, ProjectStatus::Completed) {
            project.completed_at = env.ledger().timestamp();
        }
        
        // Store updated project
        env.storage().instance().set(&key, &project);
        
        // Extend storage lifetime
        env.storage().instance().extend_ttl(5000, 5000);
        
        log!(&env, "Project {} status updated to: {:?}", project_id, new_status);
        
        return new_status;
    }
    
    // Get project details
    pub fn get_project(env: Env, project_id: u64) -> DesignProject {
        let key = ProjectBook::Project(project_id);
        
        // Retrieve and return the project
        env.storage().instance().get(&key)
            .expect("Project not found")
    }
}
