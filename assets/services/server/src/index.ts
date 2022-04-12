import Fastify from 'fastify';
import axios from 'axios';
import fs from 'fs';
import path, { ParsedPath } from 'path';

const PORT = 8080;
let IMAGE_DIRECTORY: ParsedPath;

const fastify = Fastify({ logger: true });

const downloadImage = async (
  url: string,
  imagePath: string
): Promise<boolean | Error> => {
  return axios({ url, responseType: 'stream' }).then(
    (response) =>
      new Promise((resolve, reject) => {
        response.data
          .pipe(fs.createWriteStream(imagePath))
          .on('finish', () => resolve(true))
          .on('error', (err: Error) => reject(err));
      })
  );
};

/**
 * POST /images
 * JSON: string[]
 *
 * response: boolean
 */
fastify.post('/images', async (req) => {
  const imageUrls = req.body as string[];

  const responses = imageUrls.map((url, index) => {
    const filename = url.split('/').pop() + '.jpeg' || `image_${index}`;
    return downloadImage(
      url,
      path.join(path.format(IMAGE_DIRECTORY), filename)
    );
  });

  const result = await Promise.all(responses);

  const errors: Error[] = [];

  for (const res of result) {
    if (res instanceof Error) {
      errors.push(res);
      fastify.log.error(errors);
    }
  }

  if (errors.length > 0) {
    return { status: false };
  }

  if (typeof process.send === 'function') {
    console.log('got in here');
    process.send('success');
  }

  console.log('success');
  return { status: true };
});

const main = async () => {
  const args = process.argv.slice(2);
  if (args.length < 1) {
    process.exit(1);
  }

  IMAGE_DIRECTORY = path.parse(args[0]);

  try {
    const address = await fastify.listen(PORT);
    fastify.log.info(`Server is now starting on ${address}`);
  } catch (err) {
    fastify.log.error(err);
    process.exit(1);
  }
};

main();
