package dse_example;

import org.opt4j.core.problem.ProblemModule;

public class ConfigModule extends ProblemModule {

    @Override
    protected void config() {
        bindProblem(ConfigCreator.class, ConfigDecoder.class, ConfigEvaluator.class);
    }

}
