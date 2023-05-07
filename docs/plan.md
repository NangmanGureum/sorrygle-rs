1. Make Parser
   1. How parser works
      1. Simple checker
         1. Simple Check (Abstraction from lines needed for inspection)
            1. Checking points
               1. Check channel definition
                  If the command has `--strict` option but no channel definition, It may result in an error.
               2. Check the vaildity
                  If the command doesn't have `-e`(`--extend`) option but the script has like `#B2`, It may result in an error.
      2. Main parser
        1. Clean-up
         2. Remove comments
         3. Replace some stuff to another stuff (e. g. `ABC` -> `a+b+c+`)
        2. Read stuffs and change to objects (structs)
      3. Object converter (Sorrygle -> MIDI Obj)
      4. Format export
   2. Format Structure
      1. Comment (`/= ... =/`)
      2. Master command (`((ⓚ=ⓥ))`)
      3. Channel definition (`#ⓘ` or `#{ALPHABET}ⓘ`)
         Note: This is not necessary part but if the command has `--strict` option, it may need this part.
         1. Channel command (`(ⓚ=ⓥ)`)
         2. Repeat (`|: {notes} :|ⓘ`)
         3. Note(s) (`cdefgab`)
            1. Rest note (`_`)
            2. Semitone up (`{note}+`, `{NOTE BUT UPPER CASE}`)
            3. Semitone down (`{note}-`)
            4. Octave up (`^{note}`)
            5. Octave down (`v{note}`)
         4. Multi-note
            1. Play at once (`[{notes}]`)
            2. Play at same time (`[[{notes-a}|{notes-b}]]`)
         5. Expretions
         6. Groups
            1. Group definition (`{ⓘ{notes}}`)
            2. Group indicator (`{=ⓘ}`)
2. Extended commands
   1. SysEx command
       `(sysex=f0/.../f7)`
   2. Control change command
       `(cc=ⓝ/ⓘ)`
        Parameter number is ⓝ and the value on the CC is ⓥ
   3. MIDI format reset
       `((reset={format name}))`
       (E. g. `(reset=gm)`, `(reset=gs)`, `(reset=xg)`)
   4. Multiple ports
      `#{alphabet}ⓘ`
      (E. g.`#A1`, `#E4`)
   5. Master volume
      `((mstv=ⓥ))`
   6. Master channel
      `#M`
    