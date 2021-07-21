
#  Copyright 2010-2021 Amazon.com, Inc. or its affiliates. All Rights Reserved.

#  This file is licensed under the Apache License, Version 2.0 (the "License").
#  You may not use this file except in compliance with the License. A copy of
#  the License is located at

#  http://aws.amazon.com/apache2.0/

#  This file is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
#  CONDITIONS OF ANY KIND, either express or implied. See the License for the
#  specific language governing permissions and limitations under the License.


import boto3


def execute(data_source, source_decoder, output_uri):
    client = boto3.client('emr', region_name='us-east-1')
    response = client.run_job_flow(
        Name="Spark Cluster",
        ReleaseLabel='emr-6.3.0',
        Applications=[
            {'Name': 'Spark'}
        ],
        Instances={
            'MasterInstanceType': 'm5.xlarge',
            'SlaveInstanceType': 'm5.xlarge',
            'InstanceCount': 3,
            'TerminationProtected': False,
        },
        Steps=[{
            'Name': 'Spark Application',
            'ActionOnFailure': 'CONTINUE',
            'HadoopJarStep': {
                'Jar': 'command-runner.jar',
                'Args': [
                    'spark-submit', '--deploy-mode', 'cluster',
                    's3a://XXXX/scripts/segments-assembly-0.1.jar',
                    '--data_source', data_source,
                    '--source_decoder', source_decoder,
                    '--output_uri', output_uri
                ]
            }
        }],
        VisibleToAllUsers=True,
        JobFlowRole='EMR_EC2_DefaultRole',
        ServiceRole='EMR_DefaultRole'
    )
    return response


print(execute(
    's3a://XXXX/data/source/dsgcdin-dsg_cd_20210621.csv',
    's3a://XXXX/data/decoder/Source77_Decoder.csv',
    's3a://XXXX/output/'
))
