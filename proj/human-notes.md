- Should we use GUIDs for most of the sql IDs? (except part number)?
- might need to edit the default git gnore lists in git-backedn/operation/mod.rs
- Do the db updates need to be done in the pre commit hooks? If you do it i nthe post commit will it not be commited? Or are there two commits, one for the changes and one for the metadata?
- Need to correct the part numbering int he schema, i think it;s 3-3-3 atm. Should be 2-3-6. Also the part number in the schema should just be the 6 digit number starting with 100000. The letters should be added on later as part of the humanising and printing out. 
- Tell the ai to read the project requriements (put them in the memory bank)
- time in utc
- part categories should be configurable in a cfg file
- Move to Tauri 2.0 (not sure why we are on Tauri 1.5?)
- Do we need to split some of the file into smaller ones?



# Roo Code
- Less need to update the decision log. If i is a change in the plan of record then it needs to change the POR and add a decision log. Otherwise it is just progress.
- get it to read the coding guidelines and prd every time. This may be solves in RooFlow
- Trim the progress log and context. Reccomend a run of the architect mode that will review, summarize, then archive older context. The summary stays in the main log, but we get a 8:1 summarisation and compression of progress items, or something similar?
- review the first 15k tokens that get sent.. what are they? why do they get resent on a mode change?
- for code: when refactoring a file do all the changes in one edit where possible
- Have different system prompts when switching between modes. There must be some overlap in what is sent, and there could be a shared context that gets sent at the beginning and is never sent again. We only need to send the new capabilities after a mode switch.