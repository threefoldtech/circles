# Circle-Based Collaboration System (Future Replacement for Gmail, Calendar, Docs)

## Core Concept: Circles as the Foundation

The system is built around the concept of "circles" — structured groups of people collaborating together. A circle is not individual-focused but group-focused. All data (like calendar, email, chat, docs, etc.) is shared within the circle. There’s no such thing as a personal calendar or inbox — everything lives inside a circle, a circle can be owned by 1 person though. 

### Some circle types

- Personal Circle: e.g. between two individuals
- Team/Organizational Circle: for group collaboration
- Private Circle: for individual use (e.g. digital diary, assistant)

Each circle always includes the same tools:

- Calendar
- Mail
- Chat
- Documents
- Communication e.g. chat, video conf
- AI Tools
- Other collaborative features

## Roles and Permissions

Each member of a circle has a role defining their level of access:

- Read: View-only access
- Write: Can read and modify content
- Administrator: Full control over content and membership
- Coordinator: Similar to administrator but focused on task coordination and specific permissions (e.g. managing the calendar)

These roles provide fine-grained control over who can do what inside a circle.

## Circle Data Logic

Each circle operates as a structured data model, driven by metadata and programmable rules.

- All data inside a circle is visible to each member; there are no private views within a circle.
- The circle itself is the atomic unit of collaboration and data organization.

## Names as Identity and Namespace

The system uses a naming model to identify circles, users, and entities.

Key points:

- Names always have two parts, like `project.sunflower`.
- Names are unique and can be registered.
- Name length affects cost (e.g. 2 x 8 characters = $1/year).
- Shorter or premium names may cost more.

Names act as digital identities and namespace anchors for people, circles, and data.

## Names as Records

Names can also carry metadata and function similarly to DNS records:

- Can point to IP addresses (IPv4/IPv6)
- Redirect to websites or files
- Carry TXT or custom metadata
- Be used to reference a circle or service
- Function as human-readable entry points

## Flexibility with IDs and Aliases

- Circles and users can also be referenced by UUIDs or internal IDs.
- Human-readable names make things accessible but are not required for functionality.
- Names can be linked to internal IDs and vice versa.
- Names can be changed, but may become permanent once widely shared.

## External Access and Public Sharing

- Circles can be exposed externally via their name.
- External users can be given access using a public name or shared link.
- Names can act as access tokens or public identifiers.
- Example: sending someone `event.earthsummit` to give them access to a shared space.

## Automation Layer: 

Each circle has an underlying automation engine which:

- Executes scripted logic
- Opens data models at boot time
- Handles synchronization across users
- Supports scheduled or triggered actions

This allows circles to become smart, responsive workspaces.

## Summary

This system is designed to replace existing productivity tools (like Gmail, Calendar, Docs) through:

- Circle-based model
- Namespace-based digital identity
- Role-based access control on circle level
- Structured, shared data models
- Built-in automation
- Flexible public/private sharing

Everything is unified around the concept of circles as the foundation of digital collaboration.
