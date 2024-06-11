import rc from 'rc';

const result = rc('npm', {registry: 'https://registry.npmjs.org/'});
console.info(result);