/* 
 Copyright 2019-2020 Amazon.com, Inc. or its affiliates. All Rights Reserved.
 Licensed under the Apache License, Version 2.0 (the "License"). You
 may not use this file except in compliance with the License. A copy of
 the License is located at
     http://aws.amazon.com/apache2.0/
 or in the "license" file accompanying this file. This file is
 distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF
 ANY KIND, either express or implied. See the License for the specific
 language governing permissions and limitations under the License.
*/

package com.amazon.demos

import org.apache.spark.sql.{SaveMode, SparkSession}
import org.rogach.scallop._

class Conf(arguments: Seq[String]) extends ScallopConf(arguments) {
  val data_source: ScallopOption[String] = opt[String](required = true)
  val source_decoder: ScallopOption[String] = opt[String](required = true)
  val output_uri: ScallopOption[String] = opt[String](required = true)
  verify()
}

object segments {
  def timeit[A](f: => A): A = {
    val s = System.nanoTime
    val ret = f
    println("time: " + (System.nanoTime - s) / 1e6 + "ms")
    ret
  }

  def execute(data_source: String, source_decoder: String, output_uri: String): Unit = {
    val spark = SparkSession.builder.appName("Segments").getOrCreate()
    timeit {
      spark.conf.set("log4j.logger.org.apache.spark.util.ShutdownHookManager", "OFF")
      import spark.sqlContext.implicits._

      val _decoderMap = spark.read.csv(source_decoder)
        .drop("_c1", "_c2")
        .rdd
        .map(r => (r(0).toString, r(1).toString)).collectAsMap()

      val decoderMap = spark.sparkContext.broadcast(_decoderMap)

      val source_df = spark.read.option("sep", "|")
        .csv(data_source)
        .drop("_c1", "_c2")
        .rdd
        .map(r => {
          val value: String = r(1).toString.split(",")
            .map(
              x => decoderMap.value.getOrElse(x, x)
            )
            .toSeq.mkString(",")
          (r(0).toString, value)
        }).toDF()

      source_df.write
        .mode(SaveMode.Overwrite)
        .option("header", value = false)
        .option("quoteAll", value = true)
        .csv(output_uri + java.time.LocalDateTime.now().toString)
    }
    spark.sparkContext.stop()
  }

  def main(args: Array[String]): Unit = {
    val conf = new Conf(args)
    execute(conf.data_source(), conf.source_decoder(), conf.output_uri())
  }
}