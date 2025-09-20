```mermaid
sequenceDiagram
    actor User
    participant TM as Template Manager
    participant Repository
    participant Preview as Preview Engine
    participant Export as Export Service

    %% Flow
    User ->> TM: Create template
    TM ->> Repository: Save template

    User ->> TM: Request preview
    TM ->> Preview: Generate preview
    Preview -->> TM: Return preview
    TM -->> User: Show preview

    User ->> TM: Export as PDF
    TM ->> Export: Generate PDF
    Export -->> TM: Return PDF
    TM -->> User: Deliver PDF

    %% Colors
    style User fill:#FFD580,stroke:#333,stroke-width:1px
    style TM fill:#B3E5FC,stroke:#333,stroke-width:1px
    style Repository fill:#C8E6C9,stroke:#333,stroke-width:1px
    style Preview fill:#F8BBD0,stroke:#333,stroke-width:1px
    style Export fill:#D1C4E9,stroke:#333,stroke-width:1px

