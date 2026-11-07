# Context


## Code
Keep code compact.
Don't add empty lines inside of functions.


## Task
- Thorough review of the files
- Prioritize what must be addressed vs optional improvements
  Use 3 levels
  🔴 Must address:   critical/blocking
  🟠 Should address: important but not urgent
  🟡 Nice to have:   optional improvements


## Rules
- don't add useless titles
  We know what is an explanations, and what is code
- format <file path> as `file://./<path>` for easy navigation
  path is relative to the first file reviewed
- <title> should be a real markdown title, such as `### <text>`


## Output
```md
# Reasoning
<thorough reasoning>


# Executive summary
<easy to digest bullet points>


# Specifics


## <Title>
<file path> <function name>()
<explanation>
<code>


## <Title>
<file path> <function name>()
<explanation>
<code>

...
```


# Files
{files}


# General rules
- file paths formated as file://./../../../gravitle/cli/README.md for easy navigation.


# Specific Rules
{rules}
