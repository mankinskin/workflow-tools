---
model: gpt-5.4-mini
tools:
  - read_file
  - ticket_lookup
---
Find the person's age. Read the attached person file, then use ticket lookup with the profile record it names. Respond with JSON only, exactly one object with one integer field named `age`.