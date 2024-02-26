import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

type FeatureItem = {
  title: string;
  Svg: React.ComponentType<React.ComponentProps<'svg'>>;
  description: JSX.Element;
};

const FeatureList: FeatureItem[] = [
  {
    title: 'Raspberry Pi Pico W',
    Svg: require('@site/static/img/pi-pico-w.svg').default,
    description: (
      <>
        Use the RP2040 processor, one of the most affordable and easy to use
        MCUs, on the Raspberry Pi Pico W to design and implement hardware devices.
      </>
    ),
  },
  {
    title: 'Hardware Design',
    Svg: require('@site/static/img/kicad.svg').default,
    description: (
      <>
        Learn how to use KiCad EDA to design the schematics, the 
        bill of materials (BOM) and the printed circuit board (PCB) for
        of hardware device.
      </>
    ),
  },
  {
    title: 'Embedded Rust 101',
    Svg: require('@site/static/img/rust_logo.svg').default,
    description: (
      <>
        Use Rust, a new modern and safe programming language, that is set to
        replace C/C++. Did you know that Rust is the most ❤️ language on GitHub?
        
      </>
    ),
  },
];

function Feature({title, Svg, description}: FeatureItem) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <Svg className={styles.featureSvg} role="img" />
      </div>
      <div className="text--center padding-horiz--md">
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): JSX.Element {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
