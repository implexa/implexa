- Should we use GUIDs for most of the sql IDs? (except part number)?
- might need to edit the default git gnore lists in git-backedn/operation/mod.rs
- Do the db updates need to be done in the pre commit hooks? If you do it i nthe post commit will it not be commited? Or are there two commits, one for the changes and one for the metadata?
- Need to correct the part numbering int he schema, i think it;s 3-3-3 atm. Should be 2-3-6. Also the part number in the schema should just be the 6 digit number starting with 100000. The letters should be added on later as part of the humanising and printing out. 
- Tell the ai to read the project requriements (put them in the memory bank)
- time in utc
- part categories should be configurable in a cfg file



# Roo Code
- Less need ot update the decision log
- get it to read the coding guidelines and prd every time
- trim the progress log and context
- review the first 15k tokens that get sent..