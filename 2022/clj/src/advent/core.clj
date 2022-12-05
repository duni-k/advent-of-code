(ns advent.core
  (:gen-class)
  (:require [clojure.java.io :as io]))

(defn process-file-with [f filename]
  (with-open [rdr (io/reader (io/file filename))]
    (f (line-seq rdr))))

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

;; (defn -main [& args]
;;   (if (and (seq args) (== 2 (count args)))
(println (process-file-with top-elves "day_one.txt"))
    ;; (throw (Exception. "Please pass day and part"))))

