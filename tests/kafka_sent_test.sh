#!/bin/sh

#响应Ctrl+C中断
trap 'onCtrlC' INT
function onCtrlC () {
    echo 'Ctrl+C is captured'
    exit 1
}

#kafka所在目录
kafkaPath=/usr/share/kafka
#broker
brokerlist=127.0.0.1:9092
#kafka的topic
topic=test001
#消息总数
totalNum=10
#一次批量发送的消息数
batchNum=1
#该标志为true，表示文件中的第一条记录
firstLineFlag='true'

for ((i=1; i<=${totalNum}; i ++))  
do  
	#消息内容，请按照实际需要自行调整
    messageContent=batchmessage-${i}-`date "+%Y-%m-%d %H:%M:%S"`

    #如果是每个批次的第一条，就要将之前的内容全部覆盖，如果不是第一条就追加到尾部
    if [ 'true' == ${firstLineFlag} ] ; then
      echo ${messageContent} > batchMessage.txt

      #将标志设置为false，这样下次写入的时候就不会清理已有内容了
      firstLineFlag='false'
    else
      echo ${messageContent} >> batchMessage.txt
    fi

    #取余数
    modVal=$(( ${i} % ${batchNum} ))

    #如果达到一个批次，就发送一次消息
    if [ ${modVal} = 0 ] ; then
      #在控制台显示进度
      echo “${i} of ${totalNum} sent”

      #批量发送消息，并且将控制台返回的提示符重定向到/dev/null
      cat batchMessage.txt | ${kafkaPath}/bin/kafka-console-producer.sh --broker-list ${brokerlist} --sync --topic ${topic} | > /dev/null

      #将标志设置为true，这样下次写入batchMessage.txt时，会将文件中的内容先清除掉
      firstLineFlag='true'
    fi
done
