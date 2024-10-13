Below lies the performance statistics for writing and reading operations on the SmnArchive, measured in various scenarios.

---
## Usage

To run performance tests for writing or reading forms with a specific thread count, use the following command format:
`cargo run -- test manyformsthreaded [r/w/rw/wr] [Form Count] [Thread Count]`

- **r**: Reading test
- **w**: Writing test
- **rw**: Read then Write test
- **wr**: Write then Read test

Example for writing 500 forms with 1 thread:
`cargo run -- test manyformsthreaded w 500 1`

---
## Writing Performance

The following table shows the time taken to write a specified number of forms to an archive and the average time to write one form.

|**Form Count**|**Time (s)**|**Time per Form (ms)**|
|---|---|---|
|**500 Forms**|2.2302|4.4604|
|**1000 Forms**|4.0649|4.0649|
|**5000 Forms**|21.5268|4.3054|
|**10000 Forms**|49.8846|4.9885|
|**30000 Forms**|129.3310|4.3110|
|**64000 Forms**|279.5185|4.3675|
### Write Breakdown by Percent Progress
We do binary search so the progress per percent should be more or less the same in the same sized archive.

|**Percent**|10%|20%|30%|40%|50%|60%|70%|80%|90%|100%|
|---|---|---|---|---|---|---|---|---|---|---|
|**500 Forms**|213.7225 ms|200.8740 ms|228.2313 ms|205.5635 ms|207.4483 ms|228.6233 ms|260.9252 ms|234.9282 ms|230.0272 ms|219.9679 ms|
|**1000 Forms**|396.8382 ms|397.9171 ms|404.4343 ms|397.9183 ms|389.3443 ms|457.3397 ms|416.2856 ms|406.6494 ms|392.9778 ms|405.2658 ms|
|**5000 Forms**|1990.2171 ms|2025.8925 ms|2134.7050 ms|2266.0943 ms|2099.4990 ms|2083.6680 ms|2179.8822 ms|2273.2060 ms|2314.2180 ms|2159.4578 ms|
|**10000 Forms**|5127.9978 ms|5391.6554 ms|5371.7458 ms|5744.0750 ms|5777.3128 ms|6004.3165 ms|4509.0917 ms|3981.4238 ms|3850.7895 ms|4126.2188 ms|
|**30000 Forms**|12.9932 s|14.1054 s|12.8567 s|12.9666 s|13.0252 s|12.3584 s|12.6673 s|12.9257 s|12.6532 s|12.7794 s|
|**64000 Forms**|29.2895 s|27.5282 s|25.7682 s|27.4568 s|27.7953 s|27.8274 s|27.6768 s|28.0445 s|28.6661 s|29.4657 s|

---

## Reading Performance

The following tables display the performance of reading forms from the archive using different thread counts. Each test measures the total time taken to read a specified number of forms, as well as the average time to read a single form.
### Single-Threaded Reading (1 Thread)

|**Form Count**|**Time (ms)**|**Time per Form (ms)**|
|---|---|---|
|**500 Forms**|201.1162|0.4022|
|**1000 Forms**|200.5305|0.2005|
|**5000 Forms**|501.9498|0.1004|
|**10000 Forms**|803.0814|0.0803|
|**30000 Forms**|1807.0624|0.0602|
|**64000 Forms**|3614.0523|0.0565|
### Multi-Threaded Reading (2 Threads)

|**Form Count**|**Time (ms)**|**Time per Form (ms)**|
|---|---|---|
|**500 Forms**|200.8584|0.4017|
|**1000 Forms**|200.8789|0.2009|
|**5000 Forms**|400.8351|0.0802|
|**10000 Forms**|602.2575|0.0602|
|**30000 Forms**|1405.4162|0.0468|
|**64000 Forms**|2509.1041|0.0392|
### Multi-Threaded Reading (4 Threads)

|**Form Count**|**Time (ms)**|**Time per Form (ms)**|
|---|---|---|
|**500 Forms**|200.9345|0.4019|
|**1000 Forms**|200.9285|0.2009|
|**5000 Forms**|301.2169|0.0602|
|**10000 Forms**|401.1824|0.0401|
|**30000 Forms**|1203.1943|0.0401|
|**64000 Forms**|1806.1100|0.0282|
### Multi-Threaded Reading (8 Threads)

|**Form Count**|**Time (ms)**|**Time per Form (ms)**|
|---|---|---|
|**500 Forms**|201.1985|0.4024|
|**1000 Forms**|201.0254|0.2010|
|**5000 Forms**|200.8472|0.0402|
|**10000 Forms**|301.7956|0.0302|
|**30000 Forms**|1304.0216|0.0435|
|**64000 Forms**|2510.3457|0.0392|
### Multi-Threaded Reading (16 Threads)

|**Form Count**|**Time (ms)**|**Time per Form (ms)**|
|---|---|---|
|**500 Forms**|201.4823|0.4030|
|**1000 Forms**|201.1568|0.2012|
|**5000 Forms**|200.8067|0.0402|
|**10000 Forms**|301.4653|0.0301|
|**30000 Forms**|1020.9255|0.0340|
|**64000 Forms**|2422.4692|0.0379|
