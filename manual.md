# FroxScript

FroxScript (ou apenas "FS") é uma sintaxe utilizada pelo bot do discord Frox para calcular e rolar dados. O FroxScript é um superconjunto sintático da sintaxe utilizada pelo [Rollem](https://rollem.rocks), o que significa que as expressões do Rollem também são expressões válidas de FroxScript.

## Diferenças entre o FroxScript e Rollem

-   **Performance** - O FroxScript é feito em [Rust](https://www.rust-lang.org) em conjunto com [Pest](https://pest.rs) em sua base, visando um alto desempenho, enquanto que o Rollem é feito em [Javascript](https://www.javascript.com), que é até 20 vezes mais lento.
-   **Números decimais** - O FroxScript adiciona o suporte à números com casas decimais tipo `1.41`, `3.1415`, ou `.271`.
-   **Multi-linha** - O FroxScript, diferente do Rollem, aceita várias linhas de uma vez, com cada linha contendo uma expressão.
-   **Atributos** - O FroxScript em conjunto com o Frox consegue utilizar os atributos de sua ficha no meio da expressão, facilitando as rolagens de dados. - _Nunca mais você vai ter que ficar olhando sua ficha toda vez que quer usar uma skill ou fazer um teste!_
-   **Variáveis** - Junto com os atributos, o FroxScript te permite criar valores temporários nomeados para reutilizar depois. Útil para usar um valor de uma expressão em alguma outra linha. Com isso, as possibilidades são _(quase)_ infinitas!

---

# Sintaxe

## Valores

| Sintaxe                    | Descrição                                                                                                                                                                                                                                         |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `123456`                   | Números inteiros                                                                                                                                                                                                                                  |
| `0.23` ou `.23`, `2.712`   | Números decimais                                                                                                                                                                                                                                  |
| `ATRIBUTO`, `NOME_LEGAL`   | Atributos presentes na sua ficha. Note que eles sempre devem ser escritos em letra maiúscula e só podem conter caracteres de A a Z e \_.                                                                                                          |
| `$VARIAVEL`, `$DANO_TOTAL` | Variáveis, valores temporários que tem um nome. Funcionam como atributos que não pertencem à sua ficha e são excluídos logo depois do fim da última linha. Só pode conter caracteres de A a Z e \_ além de ser obrigatório o $ no começo do nome. |

## Dados

| Sintaxe       | Descrição                                                                                                                                       |
| ------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| `df` ou `Xdf` | _(X é um número inteiro qualquer)_\ **Dado da sorte**. é um dado que tem 3 resultados: negativo, neutro ou positivo (respectivamente -, 0 e +). |
