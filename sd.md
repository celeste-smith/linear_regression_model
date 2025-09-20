# Sequence Diagram - Template Workflow (with Colors)

```mermaid
sequenceDiagram
    actor User
    participant TM as Template Manager
    participant Repository
    participant Preview as Preview Engine
    participant Export as Export Service

    rect rgb(255, 213, 128)
        User ->> TM: Create template
        TM ->> Repository: Save template
    end

    rect rgb(248, 187, 208)
        User ->> TM: Request preview
        TM ->> Preview: Generate preview
        Preview -->> TM: Return preview
        TM -->> User: Show preview
    end

    rect rgb(209, 196, 233)
        User ->> TM: Export as PDF
        TM ->> Export: Generate PDF
        Export -->> TM: Return PDF
        TM -->> User: Deliver PDF
    end


