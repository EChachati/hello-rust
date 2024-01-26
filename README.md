# hello-rust

## Variables

Las variables en rust son inmutables (no pueden modificarse) tanto en su valor como es su tipo, para hacer una variable mutable se debe agregar la palabra reservada mut y el tipo de dato se debe mantener

# Strings

Rust maneja mas de dos tipos de datos con respecto a los strings que se puede ver en su propia [documentacion](https://doc.rust-lang.org/book/ch08-02-strings.html)

## Numéricos

Los tipos de datos relacionados con numeros pueden ser definidos sin signo y con signo, es decir que se puede limitar el uso inecesario de bits en memoria como por ejemplo con una variable para la edad que no puede ser negativa y para los humanos normalente no puede superar el valor de 100 o sea que el tipo de dato mas apropiado es u8

Al manejar un número definido de bits, cada variable puede albergar hasta un cierto número de valor (Por ejemplo, si tratáramos de guardar un 256 en una variable de tipo u8, me saltaría error de Out of range (Fuera de rango)

![Alt text](images/int.png)

La diferencia entre los signed y unsigned, es que estos últimos solo utilizan su capacidad para almacenar números positivos, mientras que los signed lo usan para una cantidad igual de números positivos y negativos.

![Alt text](images/image-1.png)
![Alt text](images/image-2.png)
