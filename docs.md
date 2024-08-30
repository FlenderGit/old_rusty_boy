
Website :
- [GBDEV](https://gbdev.io/resources.html#introduction)
- [GBDEV Pandoc](https://gbdev.io/pandocs/)
- [Programming Info](https://fms.komkon.org/GameBoy/Tech/Software.html)
- [Gbdev](https://gbdev.gg8.se/wiki/articles/Interrupts)
- [Gbdev -- video display](https://gbdev.gg8.se/wiki/articles/Video_Display)

Video :
- [The Ultimate Game Boy Talk (33c3)](https://www.youtube.com/watch?v=HyzD8pNlpwI)

Opcode :
- [Pastraiser](https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
- [Meganesu](https://meganesu.github.io/generate-gb-opcodes/)

PDF : 
- [The cycle-accurate game boy docs](https://github.com/AntonioND/giibiiadvance/blob/master/docs/TCAGBD.pdf)
- [Game Boy: Complete Technical Reference](https://gekkio.fi/files/gb-docs/gbctr.pdf)

Github :
- [veikkos/chester](https://github.com/veikkos/chester)
- [mvdnes/rboy](https://github.com/mvdnes/rboy)


Calculate HC :
- 8bit  : HC = (((a & 0xF) + (b & 0xF)) & 0x10) == 0x10
- 16bit : HC = (((a & 0xFFF) + (b & 0xFFF)) & 0x1000) == 0x1000