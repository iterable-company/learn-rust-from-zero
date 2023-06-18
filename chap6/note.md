# OR
     PC                                     insts
1 ->     = split_addr

1        = split_addr   
2 ->                                 push Instruction::Split(2, 0)

ここで e1 を評価 => PCとinstsが変わる可能性あり

1       = split_addr
2
.
N ->    = jmp_addr                   push Instruction::Jump(0)

1       = split_addr
2
.
N       = jmp_addr
N+1 ->                              get_mut(split_addr).1 =>  <- 現在のPC(N+1)を代入

ここでe2を評価 => PCとinstsが変わる可能性あり

1       = split_addr
2
.
N       = jmp_addr
N+1
.
M                                   get_mut(jmp_addr).1 => <- 現在のPC(M)を代入


1                                  Split(2, N+1)
2
. e1評価
N                                  Jump(M)
N+1
. e2評価
M

# question

1 ->        = split_addr

1           = split_addr
2                                   push(Instruction::Split(2, 0))
e を評価

1           = split_addr
2
.
M                                   get_mut(split_addr).1 = M

1                                   Split(2, M)
2                                   
.   eを評価
M

# plus
## 自分の前提
1
2
.   eを評価
M                                   Split(M+1,2)

## 実際のコード
1               = l1

1               = l1
e を評価
M
M+1                                 push Instruction::Split(1, M+1)

# star
## 自分の前提
1           Split(2, M+1)
2
. eを評価
M           Split(2, M+1)
M+1

## 実際のコード
1           l1

1           l1                  push Instruction::Split(2, M+1)
2                               
. eの評価
M                               push Jump(2)
M+1                             

## *
1         Jump(1)


## 数量指定子
Split(2, M)
Decrement(idx)
. e を評価
Jump(1)