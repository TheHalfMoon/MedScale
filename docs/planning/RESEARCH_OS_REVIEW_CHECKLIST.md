# MedScale Research OS Review Checklist

Use this checklist when reviewing the planning PR.

## Product
- Does the plan serve real labs/researchers rather than only adding feature count?
- Is Projects-first organization compatible with existing MedScale workflows?
- Is AudioFlow broad enough to justify first-class status but bounded enough to implement incrementally?
- Are Personal/Lab/Institution deployments coherent parts of one product?

## Architecture
- Is there still exactly one authority plane?
- Are collaboration events separated from canonical clinical/research state?
- Can heavy services remain optional workers/adapters?
- Can indexes be rebuilt from canonical artifacts?
- Does every new plane have a clear failure and cancellation boundary?

## Privacy/security
- Can any model/agent/browser/worker receive ambient vault access?
- Can collaboration/search leak across Projects?
- Is audio capture visible and speaker identity separately governed?
- Are egress decisions external to model reasoning?
- Are remote/custom model-code paths isolated?

## Scale
- Can one-user offline operation remain simple?
- Can a lab add one Hub and GPU worker without changing object semantics?
- Can institution/HPC adapters be added without making them mandatory?
- Are scale claims deferred until measured?

## Donors
- Is each donor assigned a narrow role?
- Are Buzz and VoiceStudio prevented from defining MedScale's whole architecture?
- Are license/permission claims precise enough?
- Is wholesale copying explicitly discouraged?

## Governance
- Does the branch remain planning-only?
- Is `specs/CURRENT.md` untouched?
- Does numbering remain candidate until live reconciliation?
- Are current Product/Design truths preserved until explicitly superseded?

## Recommendation threshold

The plan is ready to merge as planning material only if reviewers can answer yes to the architecture/governance boundaries above. Merge of planning material must not be interpreted as authorization to implement Spec 074+.
