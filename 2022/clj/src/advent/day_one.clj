(ns advent.day-one)

(defn top-elves [lines]
  (reduce
   (fn [sums lines]
     (let [line (first lines)]
       (if (seq line)
         [conj
          ((+ (first sums) (Integer. line)))
          (rest sums)]
         [conj [0] sums])))
   [0]
   lines))
