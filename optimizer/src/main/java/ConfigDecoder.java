package dse_example;

import org.opt4j.core.genotype.IntegerGenotype;
import org.opt4j.core.problem.Decoder;

/**
 * A Decoder is a class used to translate between a genotype (a code-like
 * representation of problem solutions which can be easily processed and
 * modified by the GA) and a phenotype (a problem/domain - focused
 * representation of the problem which is used to evaluate the fitness of
 * individual solutions)
 * 
 */
public class ConfigDecoder implements Decoder<IntegerGenotype, Configuration> {

    @Override
    public Configuration decode(IntegerGenotype genotype) {
        return new Configuration(genotype.get(0), genotype.get(1), genotype.get(2));
    }
}
