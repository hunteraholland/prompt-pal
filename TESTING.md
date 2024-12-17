# PromptPal Test Scenarios 🧪

This document outlines various test scenarios for evaluating PromptPal's effectiveness in real-world use cases. Each test compares using PromptPal-generated context versus manual context selection.

## Test Structure
For each test scenario:
1. Document the exact prompt used
2. Record response time and token usage
3. Rate the quality of responses (1-10)
4. Note any missing context or misunderstandings
5. Track iterations needed to get desired result

## Scenarios

### 1. Code Understanding Comparison Test
**Task**: Implement a new feature in a complex codebase

**Method A - PromptPal**:
```bash
promptpal --directory ./src --xml > context.xml
```
Then use generated XML with instructions like: "I need to add a new feature to handle JSON output format. Please analyze the codebase and suggest implementation."

**Method B**: Manually select files in Cursor and add instructions

**Compare**:
- Quality of code suggestions
- Understanding of existing architecture
- Implementation correctness
- Time to get useful response

### 2. Bug Fix Efficiency Test
**Task**: Debug a complex issue spanning multiple files

**Method A - PromptPal**:
```bash
promptpal --directory ./affected_modules --xml > debug_context.xml
```

**Method B**: Manually attach error logs and relevant file snippets

**Compare**:
- Time to identify root cause
- Quality of suggested fixes
- Accuracy of problem understanding

### 3. Documentation Generation Test
**Task**: Generate comprehensive documentation for a module

**Method A - PromptPal**:
```bash
promptpal --directory ./module_to_document --xml > module_context.xml
```

**Method B**: Manually provide file contents and requirements

**Compare**:
- Documentation completeness
- Accuracy of API descriptions
- Coverage of edge cases
- Time to generate documentation

### 4. Code Review Quality Test
**Task**: Perform a thorough code review

**Method A - PromptPal**:
```bash
promptpal --directory ./pr_changes --xml > review_context.xml
```

**Method B**: Manually provide diff and files

**Compare**:
- Depth of review comments
- Issue detection rate
- Best practices suggestions
- Security concern identification

### 5. Architecture Analysis Test
**Task**: Analyze system architecture and suggest improvements

**Method A - PromptPal**:
```bash
promptpal --directory . --xml > architecture_context.xml
```

**Method B**: Manually provide high-level files and diagrams

**Compare**:
- Understanding of system relationships
- Quality of improvement suggestions
- Identification of architectural patterns
- Performance optimization ideas

### 6. API Integration Test
**Task**: Integrate a new third-party API

**Method A - PromptPal**:
```bash
promptpal --directory ./src/api --xml > api_context.xml
```

**Method B**: Manually provide relevant integration code

**Compare**:
- Code consistency with existing patterns
- Error handling completeness
- Integration best practices
- Security considerations

## Results Template

### Test Results: [Scenario Name]
- **Date**: [Test Date]
- **PromptPal Version**: [Version]
- **LLM Used**: [e.g., GPT-4, Claude, etc.]

#### Method A (PromptPal)
- Time to prepare context: [minutes]
- Token count: [number]
- Response quality (1-10): [rating]
- Iterations needed: [number]
- Notable observations: [text]

#### Method B (Manual)
- Time to prepare context: [minutes]
- Token count: [number]
- Response quality (1-10): [rating]
- Iterations needed: [number]
- Notable observations: [text]

#### Conclusion
[Summary of findings and recommendations] 