import winston from 'winston';
import { KafkaClient, Producer } from 'kafka-node';

class KafkaTransport extends winston.Transport {
  constructor(opts) {
    super(opts);

    this.client = new KafkaClient({ kafkaHost: opts.kafkaHost || process.env.KAFKA_HOST });
    this.producer = new Producer(this.client);

    this.topic = opts.topic || 'logs';
    this.producer.on('ready', () => console.log('Kafka producer is ready'));
    this.producer.on('error', (err) => console.error('Kafka producer error:', err));
  }

  log(info, callback) {
    const message = JSON.stringify({
      ...info
    });

    const payloads = [{ topic: this.topic, messages: message }];

    // Send the log to Kafka
    this.producer.send(payloads, (err) => {
      if (err) {
        console.error('Error sending log to Kafka:', err);
      }

      if (callback) callback();
    });
  }
}

export default KafkaTransport;
