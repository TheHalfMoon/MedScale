# MedScale Research OS Architecture Review Prompts

Before approving future architecture, challenge it with these questions:

- What is the canonical authority for this state?
- What happens with no network?
- What exact data leaves the local trust boundary?
- Can a model/agent/tool cause this action without Core policy?
- What evidence explains the result six months later?
- How is cancellation represented?
- How does the system recover from a crash halfway through?
- What is the smallest donor/dependency surface that solves the requirement?
- Can the index/cache be deleted and rebuilt?
- Can an institutional deployment scale this without changing the object semantics?
- Does this feature belong in Core, a Pack, a worker, an adapter, or nowhere?
- What user journey becomes materially better?

A design that cannot answer these questions should not be promoted because it is visually compelling or available in a donor project.
