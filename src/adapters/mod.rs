// Adapters Module - Bridge between NEW and OLD architectures
//
// This module provides adapters that translate between the NEW architecture (V3.0)
// and the OLD architecture (V2.0) physical manifestation system.
//
// From COMPREHENSIVE_REFACTOR_PLAN.md Phase 4:
// "Create adapter that connects NEW architecture to OLD physical system"
//
// Key Principles:
// 1. Adapter layer should be thin and focused
// 2. Translation mappings should be well-documented
// 3. Backward compatibility maintained during migration
// 4. Eventually replaced by NEW physical system

pub mod physical_adapter;

// Re-export main types for convenience
