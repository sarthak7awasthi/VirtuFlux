# VirtuFlux Project

**VirtuFlux** is a distributed system designed for dynamic data routing, processing, and resource management across virtual nodes. It aims to ensure efficient and scalable data flow management, load balancing, fault tolerance, and distributed processing, using virtual machines (VMs) in a dynamic environment. The project leverages KVM/QEMU for virtualization and incorporates a range of algorithms to ensure system resilience, high availability, and optimized resource usage.

## Key Features

- **Dynamic Data Routing**: The system dynamically routes data to the most suitable virtual node based on real-time performance metrics, preventing overload and optimizing processing.
  
- **Load Balancing**: Real-time monitoring of system load (CPU, memory, network usage) is used to distribute workloads evenly across virtual nodes, ensuring that no single node becomes a bottleneck.

- **Fault Tolerance**: The system includes mechanisms to detect node failures and automatically redistributes tasks to operational nodes, ensuring uninterrupted data processing.

- **Data Partitioning & Replication**: Large datasets are partitioned into smaller chunks for parallel processing. Data replication ensures that even if a node fails, there is no loss of critical data.

- **Scalable Virtualized Environment**: The project uses KVM/QEMU for managing virtual nodes, enabling efficient resource allocation and seamless communication between nodes in the distributed system.

## Architecture Overview

VirtuFlux is designed in a modular architecture, where each module focuses on a specific responsibility. Below is a high-level overview of the modules and their functions:


![image](https://github.com/user-attachments/assets/a4d69010-6925-4993-aa17-17b3dc674510)



### 1. Virtualization Management Module
   - **Objective**: Handles the creation, management, and destruction of virtual machines (VMs).
   - **Key Functions**:
     - `create_vm`: Creates a new VM with specified CPU, memory, disk, and network configurations.
     - `start_vm`: Starts an existing VM.
     - `stop_vm`: Stops a running VM.
     - `destroy_vm`: Destroys a VM and releases allocated resources.
     - `list_vms`: Lists all the VMs with their current status and resource usage.

### 2. Dynamic Routing Module
   - **Objective**: Routes data packets to the appropriate VM based on real-time performance and routing algorithms.
   - **Key Functions**:
     - `route_data`: Routes a data packet to the most suitable VM based on the current system load and routing algorithm.
     - `configure_routing_algorithm`: Configures the routing algorithm to optimize data flow (e.g., round-robin, least load).
     - `monitor_routing_performance`: Monitors routing performance and optimizes future decisions based on real-time metrics.

### 3. Load Balancing Module
   - **Objective**: Distributes the workload evenly across VMs to prevent bottlenecks and ensure efficient resource usage.
   - **Key Functions**:
     - `calculate_load`: Calculates the current load on a VM using performance metrics like CPU and memory usage.
     - `distribute_load`: Distributes incoming data across available VMs, factoring in their current load.
     - `configure_load_balancing_algorithm`: Configures the algorithm for load balancing (e.g., least load, round-robin).

### 4. Fault Tolerance Module
   - **Objective**: Ensures system stability and continuous operation in case of node failures.
   - **Key Functions**:
     - `detect_node_failure`: Detects if a VM is unresponsive or has failed.
     - `initiate_failover`: Redistributes tasks from failed nodes to operational ones.
     - `recover_data`: Recovers lost data from replica nodes in case of node failure.
     - `health_check`: Performs routine checks to monitor the health status of all VMs.

### 5. Data Partitioning and Replication Module
   - **Objective**: Breaks down large datasets for parallel processing and ensures fault tolerance by replicating critical data across nodes.
   - **Key Functions**:
     - `partition_data`: Partitions large datasets into smaller chunks for parallel processing across VMs.
     - `replicate_data`: Replicates data chunks across multiple VMs to ensure fault tolerance.
     - `check_data_consistency`: Verifies that replicated data across nodes is consistent and synchronized.
     - `merge_data_chunks`: Reassembles partitioned data chunks into a complete dataset after processing.

### 6. Monitoring and Analytics Module
   - **Objective**: Continuously monitors system performance and provides real-time analytics for decision-making.
   - **Key Functions**:
     - `collect_metrics`: Gathers performance metrics from each VM (CPU, memory, network).
     - `aggregate_metrics`: Aggregates collected metrics into a holistic view of system performance.
     - `trigger_alert`: Issues alerts when performance thresholds are exceeded (e.g., high CPU usage).
     - `generate_report`: Creates detailed reports on system performance over a given time period.

### 7. Control Plane Module
   - **Objective**: Central management layer for coordinating the actions of all modules and providing a user interface for interaction.
   - **Key Functions**:
     - `manage_vm_lifecycle`: Manages the lifecycle of VMs (create, start, stop, destroy).
     - `coordinate_routing_and_balancing`: Coordinates routing and load balancing activities between the relevant modules.
     - `handle_alerts_and_failovers`: Manages system alerts and triggers failover processes as needed.
     - `expose_api`: Provides a RESTful API for external systems or users to interact with VirtuFlux.

## Data Types & Structures

- **Data Packets**: These are the core units of data handled by the system.
  - **Structure**:
    - `packet_id`: Unique identifier for the packet.
    - `data`: The actual payload to be processed.
    - `priority`: Priority level (0-255) to indicate the importance of processing.
    - `timestamp`: Timestamp of the packet creation.
  
- **Large Dataset**: Represents big data workloads such as logs, sensor data, or large files.
  - **Structure**:
    - `dataset_id`: Unique identifier for the dataset.
    - `data`: Raw dataset in binary or text format.
    - `partition_strategy`: Strategy for partitioning (e.g., size-based, round-robin).

- **Performance Metrics**: Data collected from VMs for load balancing and fault tolerance.
  - **Structure**:
    - `vm_id`: Unique identifier for the VM.
    - `cpu_usage`: CPU usage percentage.
    - `memory_usage`: Memory usage in MB.
    - `network_usage`: Network throughput in bytes.
    - `uptime`: Time since the VM started, in seconds.

- **Replicated Data Chunks**: Partitioned datasets replicated across VMs for redundancy.
  - **Structure**:
    - `chunk_id`: Unique identifier for the data chunk.
    - `original_vm_id`: VM that originally processed/stored the chunk.
    - `replica_vm_ids`: List of VMs storing replicas of the chunk.

**Note:**
This is a very basic version created just for my personal learning, and is still incomplete.
This README serves as an overview of the project’s goals, architecture, and key components.
