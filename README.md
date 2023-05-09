# Rust-Fundamentals
Doing a rust course, initial rusty


<p align="center">
  <img src= https://ithelp.ithome.com.tw/upload/images/20190923/20119807TBGJ1hynCh.png width="350">
</p>

Al manejar un número definido de bits, cada variable puede albergar hasta un cierto número de valor (Por ejemplo, si tratáramos de guardar un 256 en una variable de tipo u8, me saltaría error de Out of range (Fuera de rango)

La diferencia entre los signed y unsigned, es que estos últimos solo utilizan su capacidad para almacenar números positivos, mientras que los signed lo usan para una cantidad igual de números positivos y negativos.

<p align = "center">
 <img src= https://miro.medium.com/max/1400/1*MxVEixCs1iS1shQs2JVTYg.png width="550">
</p>

<p align = "center">
 <img src= https://miro.medium.com/max/1400/1*ScXl3GI_8EY0Ow4t-1dRUg.png width="550">
</p>



Que es unwrap? / Manejo de Errores
- Se usa Unwrap cuando estamos seguros que no habra ningun error
- En rust no existe el null;
- Si un valor puede o no existir se usa Option<T> , siendo T el tipo base.
- En rust los errores son un tipo de dato.
- Si una respuesta puede devolver error, sera del tipo Result<T,E>, siendo T el tipo sin error y E el error dado
- Discusion de stackOverflow para usar unwrap de la mejor manera

 
  
