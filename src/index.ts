const arr = [1, 7, 2, 3, 9];

function insertionSort() {
  for (let i = 1; i < arr.length; i++) {
    let cur = arr[i];
    let j = i - 1;
    while (j >= 0 && arr[j] > arr[i]) {
      arr[j + 1] = arr[j];
      j = j - 1;
    }
    arr[j + 1] = cur;
  }
}

insertionSort();
console.log(arr);
