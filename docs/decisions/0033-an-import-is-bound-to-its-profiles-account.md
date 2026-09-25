---
id: ADR-0033
status: accepted
date: 2026-09-26
area: persistence
supersedes: []
superseded-by: []
related: [ADR-0002, ADR-0031, ADR-0032]
---

# ADR-0033: An import that states an account is checked against its profile's account, in store format 3

## Status

Accepted, 2026-09-26, on the maintainer's instruction. It replaces ADR-0032,
rule 8, and moves the store format from 2 to 3. ADR-0032's other rules stand,
and it carries an amendment pointing here. ADR-0032, rule 8, was **right at the
time and overtaken by circumstances**: when it was written the snapshot IR
stated no account, so there was nothing to check. The IR now does (ADR-0031,
`spec/snapshot-ir.md`).

## Context

A profile is one game account. Importing another account's file into it would
mix two inventories, and the fold would accept it silently. Store format 1
refused such an import: an acquisition carried the account it was read from, and
the fold compared it with the account the profile knew. ADR-0032 dropped that
check because the IR had no account to compare.

The IR now has `account: Option<AccountRef>`. A `yata-snapshot` file may state
it. The first community format's adapter does not fill it, because its account
field's meaning is not established. So some imports state an account and many do
not.

## Decision

1. **`SnapshotImported` records the account the snapshot states**, or none.

   ```text
   SnapshotImported { snapshot, original, source, sections, account : GameAccountId? }
   ```

2. **The profile's known account is derived, never stored as a fact of its
   own.**

   ```text
   known(p) = the account of p's ProfileCreated,                       if it names one
            = the account of p's earliest current import that states one, otherwise
            = none,                                                    if neither exists
   ```

   "Current" means not withdrawn. A withdrawn import no longer decides the
   account, so withdrawing the import that bound a profile frees the profile to
   be bound by the next one. A wrongly imported file can therefore be undone.

3. **An import that states an account is checked; one that states none is not.**

   ```text
    import states a    known(p) = k    a ≠ k
   ─────────────────────────────────────────── (Mismatch)
    the commit does not apply: ProfileMismatch { seq, profile, known: k, observed: a }

    import states a    known(p) = none
   ─────────────────────────────────────────── (Bind)
    the commit applies; known(p) = a from now on, while this import is current

    import states no account
   ─────────────────────────────────────────── (Unchecked)
    the commit applies; known(p) is unchanged
   ```

   The check is in the fold, so the rule that refuses an import is the rule that
   would refuse a log holding it (`fact-format.md`, § Commits and sequence
   numbers). Nothing is written for a refused import.

4. **Store format 3.** The import fact gained a field, which moves the store
   format (ADR-0032, rule 2). A format-2 store is retired like a format-1 store
   (ADR-0032, rule 3): refused with `RetiredFormat`, and recreated. No released
   build wrote one.

## Alternatives

- **Bind only at profile creation.** Rejected: most profiles are created before
  the user knows which file will state the account, and a community file states
  none today.
- **Store the binding as its own fact, `ProfileBound`.** Rejected: it is a
  second encoding of what the imports already say, and it would need its own
  retraction rule. Derived, the binding follows retractions for free.
- **Bind to the latest import instead of the earliest.** Rejected: a later file
  of another account would then rebind the profile instead of being refused.
- **Refuse an import that states no account once the profile knows one.**
  Rejected: the first community format never states one, so every such import
  would be refused.
- **Lift format 2 to format 3.** Rejected for the reason ADR-0032 gives for
  format 1: no released build wrote one, and a developer's store is recreated.

## Consequences

**Easier.** A file of another account is refused before anything is written, and
a wrong import is undone by retracting it.

**Harder.** An import without an account is never checked, so a community file
of another account still lands. That closes only when its format's adapter
states the account. Every developer store is recreated a second time.

**Records that change with this one.** `spec/fact-format.md` (the import fact,
the account rule, format 3), `project/status.md`, `evidence/testing.md`, a
change fragment, and an amendment to ADR-0032.
