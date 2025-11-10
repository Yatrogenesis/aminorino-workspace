//! Real-time monitoring and telemetry system
//!
//! Implements comprehensive quantum system monitoring with nanosecond-precision
//! timestamps and full quantum state tracking.

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Quantum event type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuantumEventType {
    /// Gate execution
    GateExecution,
    /// Measurement
    Measurement,
    /// Decoherence event
    Decoherence,
    /// Error correction triggered
    ErrorCorrection,
    /// Radiation event
    RadiationEvent,
    /// Platform-specific event
    PlatformEvent,
}

/// Quantum event with full telemetry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumEvent {
    /// Timestamp in nanoseconds since UNIX epoch
    pub timestamp_ns: u128,
    /// Event type
    pub event_type: QuantumEventType,
    /// Affected qubit IDs
    pub qubit_ids: Vec<usize>,
    /// Gate fidelity (if applicable)
    pub fidelity: Option<f64>,
    /// Error syndrome (if applicable)
    pub syndrome: Option<Vec<bool>>,
    /// Additional metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl QuantumEvent {
    /// Create a new quantum event
    pub fn new(event_type: QuantumEventType, qubit_ids: Vec<usize>) -> Self {
        let timestamp_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_nanos();

        Self {
            timestamp_ns,
            event_type,
            qubit_ids,
            fidelity: None,
            syndrome: None,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Add fidelity information
    pub fn with_fidelity(mut self, fidelity: f64) -> Self {
        self.fidelity = Some(fidelity);
        self
    }

    /// Add syndrome information
    pub fn with_syndrome(mut self, syndrome: Vec<bool>) -> Self {
        self.syndrome = Some(syndrome);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Telemetry system
#[derive(Debug)]
pub struct TelemetrySystem {
    /// Event buffer
    events: Vec<QuantumEvent>,
    /// Maximum buffer size
    max_buffer_size: usize,
    /// Total events recorded
    total_events: u64,
}

impl TelemetrySystem {
    /// Create a new telemetry system
    pub fn new(max_buffer_size: usize) -> Self {
        Self {
            events: Vec::with_capacity(max_buffer_size),
            max_buffer_size,
            total_events: 0,
        }
    }

    /// Record an event
    pub fn record(&mut self, event: QuantumEvent) {
        if self.events.len() >= self.max_buffer_size {
            // FIFO rotation
            self.events.remove(0);
        }

        self.events.push(event);
        self.total_events += 1;
    }

    /// Get recent events
    pub fn recent_events(&self, count: usize) -> &[QuantumEvent] {
        let start = self.events.len().saturating_sub(count);
        &self.events[start..]
    }

    /// Get all events
    pub fn all_events(&self) -> &[QuantumEvent] {
        &self.events
    }

    /// Clear buffer
    pub fn clear(&mut self) {
        self.events.clear();
    }

    /// Get total events recorded (including rotated out)
    pub fn total_events(&self) -> u64 {
        self.total_events
    }

    /// Calculate average fidelity over recent events
    pub fn average_fidelity(&self, event_type: QuantumEventType) -> Option<f64> {
        let relevant_events: Vec<f64> = self
            .events
            .iter()
            .filter(|e| e.event_type == event_type)
            .filter_map(|e| e.fidelity)
            .collect();

        if relevant_events.is_empty() {
            None
        } else {
            Some(relevant_events.iter().sum::<f64>() / relevant_events.len() as f64)
        }
    }

    /// Detect performance bottlenecks
    pub fn detect_bottlenecks(&self) -> Vec<String> {
        let mut bottlenecks = Vec::new();

        // Check average gate fidelity
        if let Some(avg_fidelity) = self.average_fidelity(QuantumEventType::GateExecution) {
            if avg_fidelity < 0.95 {
                bottlenecks.push(format!(
                    "Low gate fidelity: {:.4} (threshold: 0.95)",
                    avg_fidelity
                ));
            }
        }

        // Check for frequent error corrections
        let error_correction_count = self
            .events
            .iter()
            .filter(|e| e.event_type == QuantumEventType::ErrorCorrection)
            .count();

        if error_correction_count > self.events.len() / 10 {
            bottlenecks.push(format!(
                "High error correction rate: {} events in last {} total",
                error_correction_count,
                self.events.len()
            ));
        }

        // Check for radiation events
        let radiation_count = self
            .events
            .iter()
            .filter(|e| e.event_type == QuantumEventType::RadiationEvent)
            .count();

        if radiation_count > 0 {
            bottlenecks.push(format!(
                "Radiation events detected: {} in recent history",
                radiation_count
            ));
        }

        bottlenecks
    }
}

impl Default for TelemetrySystem {
    fn default() -> Self {
        Self::new(10000) // Default 10k events buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_event_creation() {
        let event = QuantumEvent::new(QuantumEventType::GateExecution, vec![0, 1])
            .with_fidelity(0.99);

        assert_eq!(event.event_type, QuantumEventType::GateExecution);
        assert_eq!(event.qubit_ids, vec![0, 1]);
        assert_eq!(event.fidelity, Some(0.99));
    }

    #[test]
    fn test_telemetry_recording() {
        let mut telemetry = TelemetrySystem::new(5);

        for i in 0..10 {
            let event = QuantumEvent::new(QuantumEventType::GateExecution, vec![i]);
            telemetry.record(event);
        }

        assert_eq!(telemetry.all_events().len(), 5); // FIFO kept only 5
        assert_eq!(telemetry.total_events(), 10); // But counted all 10
    }

    #[test]
    fn test_average_fidelity() {
        let mut telemetry = TelemetrySystem::new(100);

        telemetry.record(
            QuantumEvent::new(QuantumEventType::GateExecution, vec![0]).with_fidelity(0.99),
        );
        telemetry.record(
            QuantumEvent::new(QuantumEventType::GateExecution, vec![1]).with_fidelity(0.97),
        );

        let avg = telemetry
            .average_fidelity(QuantumEventType::GateExecution)
            .unwrap();
        assert!((avg - 0.98).abs() < 0.01);
    }

    #[test]
    fn test_bottleneck_detection() {
        let mut telemetry = TelemetrySystem::new(100);

        // Add events with low fidelity
        for _ in 0..5 {
            telemetry.record(
                QuantumEvent::new(QuantumEventType::GateExecution, vec![0]).with_fidelity(0.90),
            );
        }

        let bottlenecks = telemetry.detect_bottlenecks();
        assert!(!bottlenecks.is_empty());
        assert!(bottlenecks[0].contains("Low gate fidelity"));
    }
}
