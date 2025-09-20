```mermaid
%%{init: {"theme": "default", "themeVariables": { "fontSize": "14px"}, "sequence": {"showSequenceNumbers": false}, "themeCSS": ".mermaidTooltip, .mermaid .interactive { display:none !important; }"}}%%
sequenceDiagram
    actor User
    participant TM as Template Manager
    participant Repository
    participant Preview as Preview Engine
    participant Export as Export Service

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

