# FroxScript

FroxScript (ou apenas "FS") é uma sintaxe utilizada pelo bot do discord Frox para calcular e rolar dados. O FroxScript é um superconjunto sintático da sintaxe utilizada pelo [Rollem](https://rollem.rocks), o que significa que as expressões do Rollem também são expressões válidas de FroxScript.

## Diferenças entre o FroxScript e Rollem

-   **Performance** - O FroxScript é feito em [Rust](https://www.rust-lang.org) em conjunto com [Pest](https://pest.rs) em sua base, visando um alto desempenho. Enquanto que o Rollem é feito em [Javascript](https://www.javascript.com), que é até 20 vezes mais lento.
-   **Números decimais** - O FroxScript adiciona o suporte à números com casas decimais tipo `1.41`, `3.1415`, ou `.271`.
-   **Multi-linha** - O FroxScript, diferente do Rollem, aceita várias linhas de uma vez, com cada linha contendo uma expressão.
-   **Atributos** - O FroxScript em conjunto com o Frox consegue utilizar os atributos de sua ficha no meio da expressão, facilitando as rolagens de dados. - _Nunca mais você vai ter que ficar olhando sua ficha toda vez que quer usar uma skill ou fazer um teste!_
-   **Variáveis** - Junto com os atributos, o FroxScript te permite criar valores temporários nomeados para reutilizar depois. Útil para usar um valor de uma expressão em alguma outra linha. Com isso, as possibilidades são _(quase)_ infinitas!
- **Mais operadores**

---

# Sintaxe

## Valores

| Sintaxe | Descrição |
| --- | --- |
| `123456` | Números inteiros. |
| `0.23` ou `.23`, `2.712` | Números decimais. |
| `ATRIBUTO`, `NOME_LEGAL` | Atributos presentes na sua ficha. Note que eles sempre devem ser escritos em letra maiúscula e só podem conter caracteres de A a Z e \_. |
| `$VARIAVEL`, `$DANO_TOTAL` | Variáveis, valores temporários que tem um nome. Funcionam como atributos que não pertencem à sua ficha e são excluídos logo depois do fim da última linha. Só é possivel usar uma variável se ela foi [definida](#atribuição) antes. Só pode conter caracteres de A a Z e \_ além de ser obrigatório o $ no começo do nome. |
| Dados | Ver [Dados](#dados).
| `(expressão)` | É possível envolver uma expressão em volta de parênteses para aumentar a sua prioridade. |



## Dados
- *Para os exemplos, `X` e `Y` são números inteiros.*
  1. *X tem que ser entre 1 e 100.*
  2. *Y não pode ser menor que 2.*

| Sintaxe | Descrição |
| --- | --- |
| `df` ou `Xdf` | **Dado da sorte**. É um dado que tem 3 resultados: negativo, neutro ou positivo (representados por -, 0 e +). <br /> `X` determina a quantidade de dados na rolagem (se omitido, é 1) <br /> Exemplo: `2df`|
| `dY` ou `XdY` | **Dado comum**. Roda `X` dados de `Y` lados (ou seja, valores entre 1 e `Y`). Se `X` for omitido, ele é considerado como 1. <br /> Exemplo: `4d20` |



## Opções de Dados
> Essas opções servem para modificar uma rolagem.
- *`[dado]` é um [Dado comum](#dados).*
- *Um mesmo dado pode ter várias dessas opções, mas `k`, `d`, `kh`, `kl`, `dh`, `dl` e `c` não são compatíveis entre si.*
- *Para os exemplos, `X` e `Y` são números inteiros.*
  1. *X tem que ser entre 1 e a quantidade de dados do `[dado]`.*
  2. *Y tem que ser entre 1 e o número de lados do `[dado]`.*

| Sintaxe | Descrição |
| --- | --- |
| ` [dado]! ` ou ` [dado]!X ` | **Explosão**. Para cada dado que deu `X` ou maior, rola um dado adicional. Se `X` for omitido, o valor dele é igual ao número de lados do `[dado]`. <br /> A chance de explodir não pode ser maior que 99.99%. <br /> Exemplo: ` d6! ` |
| ` [dado]aro ` | **Iguais**. Se todos os dados caírem em números iguais, rola um dado adicional. Se esse dado adicional também cair no mesmo número, repete o processo rolando mais outro dado. <br /> Exemplo: ` 2d6aro ` |
| ` [dado]s ` | **Ordenar**. Ordena os dados de ordem decrescente <br /> Essa opção já é inclusa ao utilizar ` k `, ` d `, ` kh `, ` kl `, ` dh ` ou ` dl `. <br /> Exemplo: ` 20d20s ` |
| ` khX ` ou ` kX ` | **Manter maiores**. Mantém os `X` maiores dados e descarta o resto. <br /> Exemplo: ` 2d20k1 ` |
| ` klX ` | **Manter menores**. Mantém os `X` menores dados e descarta o resto. <br /> Exemplo: ` 5d6kl3 ` |
| ` dlX ` ou ` dX ` | **Descartar menores**. Descarta os `X` menores dados e mantém o resto. <br /> Exemplo: ` 5d6d4 ` |
| ` dhX ` | **Descartar maiores**. Descarta os `X` maiores dados e mantém o resto. <br /> Exemplo: ` 10d20dh5 ` |
| ` cY ` | **Crítico**. Considera os dados com o valor de `Y` ou maior como crítico. (Efeito apenas visual) <br /> Exemplo: ` 1d20c15 ` |



## Operadores binários
> São operadores que tomam dois valores.
- _`A` e `B` são expressões quaisquer._

### Comuns
- _`T` e `U` são testes quaisquer._

| Sintaxe | Descrição |
| --- | --- |
| ` A + B ` | **Soma**. <br /> Exemplo: ` 20 + 4 ` |
| ` A - B ` | **Subtração**. <br /> Exemplo: ` 20 - 4 ` |
| ` A * B ` | **Multiplicação**. <br /> Exemplo: ` 12 * 2 ` |
| ` A / B ` | **Divisão**. <br /> Exemplo: ` 48 / 2 ` |
| ` A < B ` | **Menor**. Cria um teste que é verdadeiro quando `A` é menor que `B`. <br /> Exemplo: ` 24 < 48 ` |
| ` A <= B ` | **Menor igual**. Cria um teste que é verdadeiro quando `A` é menor ou igual a `B`. <br /> Exemplo: ` 12 <= 12` |
| ` A > B ` | **Maior**. Cria um teste que é verdadeiro quando `A` é maior que `B`. <br /> Exemplo: ` d20 > 16 ` |
| ` A >= B ` | **Maior igual**. Cria um teste que é verdadeiro quando `A` é maior ou igual a `B`. <br /> Exemplo: ` d20 >= 10 ` |
| ` A = B ` | **Igual**. Cria um teste que é verdadeiro quando `A` é igual a `B`. <br /> Exemplo: `2 = 1 + 1 ` |
| ` T & U ` | **Ambos**. Cria um teste quando ambos os testes `T` e `U` são verdadeiros. <br /> Exemplo: ` (1 < 2) & (3 < 4) ` |
| ` T \| U ` | **Ou**. Cria um teste quando ambos os testes `T` ou `U` são verdadeiros. <br /> Exemplo: ` (1 = 10) \| (2 <= 4) ` |

### Operadores de dados
- _`D` é uma rolagem de dados._

| Sintaxe | Descrição |
| --- | --- |
| ` D ++ B ` | **Soma interna**. Soma cada dado de `D` com o valor de `B`. <br /> Exemplo: ` 5d10 ++ 5 ` |
| ` D -- B ` | **Subtração interna**. Subtrai cada dado de `A` com o valor de `B`. <br /> Exemplo: ` 6d10 -- 2 ` |
| ` D << B ` | **Contador (menor)**. Conta quantos dados de `D` são iguais ou menores a `B`. <br /> Exemplo: ` 50d20 << 10 ` |
| ` D >> B ` | **Contador (maior)**. Conta quantos dados de `D` são iguais ou maiores a `B`. <br /> Exemplo: ` 50d20 >> 10 ` |



## Operadores unários
> São operadores que tomam apenas um valor. Dependendo do operador, ele fica na esquerda ou na direita.
- `A` é um valor qualquer.
- `T` é um teste.

### Esquerda
| Sintaxe | Descrição |
| --- | --- |
| ` -A ` | **Negativo**. Inverte o sinal do número. <br /> Exemplo: ` -20 ` |
| ` !T ` | **Inverter**. Inverte o resultado de um teste. Se ele tinha dado "Sucesso!", vira "Falha!" e vice-versa. <br /> Exemplo: ` !(2 > 20) ` = "Sucesso!" |
| ` ^A ` | **Arredondar para cima**. <br /> Exemplo: ` ^10.2 ` = 11 |
| ` ~A ` | **Arredondar para o mais próximo**. <br /> Exemplo: ` ~10.2 ` = 10 / `~10.7` = 11 |
| ` _A ` | **Arredondar para baixo**. <br /> Exemplo: ` _10.7 ` = 10 |

### Direita
| Sintaxe | Descrição |
| --- | --- |
| ` A% ` | **Porcentagem**. Transforma o número em uma porcentagem. (Tem o mesmo efeito de dividir por 100). <br /> Exemplo: ` 230% ` = 2.3 |



## Atribuição
- _`E` é uma expressão qualquer._
- _`ATR` é um atributo qualquer._ 
- _`$VAR` é uma variável qualquer._ 

| Sintaxe | Descrição |
| --- | --- |
| ` ATR := E ` ou ` $VAR := E `| **Atribuição**. Atribui o valor da expressão `E` para o atributo/variável `ATR`/`$VAR`. No caso da variável, se ela não tiver sido criada antes, ela é criada automaticamente. Só pode haver uma atribuição por linha. <br /> Exemplo: ` ATRIBUTO := 20 ` / ` $VAR := 2d20 ++ 20 ` |



## Comentário
> Ao colocar um ` ; ` em uma linha, qualquer texto depois dele será ignorado e tratado como comentário.



## Avançado
- Coalescência de Tipagem
  - **Se tiver um valor comum no lugar de um teste em operadores de teste como ` > `, ` >= `, ` < `, ` <= `, ` = `,  ` | ` e ` & `:** Ele será transformado em um teste. Se ele for igual a zero ele se tornará `Falha!`, caso contrário será `Sucesso!`.
  - **Se tiver um teste no lugar de um valor em operações normais:** Ele será transformado em um número. Se for `Verdadeiro!` se tornará `1`, se for `Falha!` será `0`.
