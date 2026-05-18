import init, { run_app } from './pkg/index.js';

async function bootstrap() {
    await init();
    run_app();
}

bootstrap();
