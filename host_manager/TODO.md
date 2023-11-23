# Coverage based 
- Create Input queue
 - new coverage -> save input to queue
 - check packet is deterministic 
  - if it is, save mutated packet in hashed input packet folder (key (input packet) - value (mutated packet))
  - if it is not, umm... classify using stage?
   - or mabye classify using client's opcode? becuase other optionses are alwayes same...?
   - then, some variable is depend to client's value. So, need to use offset based mutate (save {offset:change_value}[..] instead of mutated_value[..])

