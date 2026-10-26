pub fn selection<T: PartialOrd>(arr: &mut [T]) -> &mut [T] {
   let n = arr.len();

    for i in 0..n {

        let mut min_index = i;

        for j in (i + 1)..n {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }

        if min_index != i {
            arr.swap(i, min_index);
        }
    }
    arr
}

pub fn insertion<T: PartialOrd>(arr: &mut [T]) -> &mut [T] {

    let n = arr.len();

    for i in 1..n {
        let mut j = i;

        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }

    arr
}

pub fn bubble<T: PartialOrd>(arr: &mut [T]) -> &mut [T] {
    let n = arr.len();

    for i in 0..n {
        let mut trocou = false;

        for j in 0..(n - 1 - i) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                trocou = true;
            }
        }

        if !trocou {
            break;
        }
    }

    arr
}

pub fn merge<T: PartialOrd + Clone>(arr: &mut [T]) -> &mut [T] {
    let n = arr.len();

    if n <= 1 {
        return arr;
    }

    let meio = n / 2;

    // Ordena cada metade recursivamente
    {
        let (esq, dir) = arr.split_at_mut(meio);
        merge(esq);
        merge(dir);
    }

    // Copia as metades já ordenadas para vetores auxiliares
    let esq = arr[..meio].to_vec();
    let dir = arr[meio..].to_vec();

    let (mut i, mut j, mut k) = (0, 0, 0);

    // Intercala as duas metades de volta em arr
    while i < esq.len() && j < dir.len() {
        if esq[i] <= dir[j] {
            arr[k] = esq[i].clone();
            i += 1;
        } else {
            arr[k] = dir[j].clone();
            j += 1;
        }
        k += 1;
    }

    // Sobras de alguma das metades
    while i < esq.len() {
        arr[k] = esq[i].clone();
        i += 1;
        k += 1;
    }

    while j < dir.len() {
        arr[k] = dir[j].clone();
        j += 1;
        k += 1;
    }

    arr
}

