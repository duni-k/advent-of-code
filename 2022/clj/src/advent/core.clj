(ns advent.core
  (:gen-class)
  (:require [clojure.java.io :as io]
            [advent.day-one :as one]))

(defn process-file-with [f filename]
  (with-open [rdr (io/reader (io/file filename))]
    (f (line-seq rdr))))

;; (defn -main [& args]
;;   (if (and (seq args) (== 2 (count args)))
(println (process-file-with one/top-elves "day_one.txt"))
    ;; (throw (Exception. "Please pass day and part"))))
