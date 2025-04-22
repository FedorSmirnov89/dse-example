package dse_example;

import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ExecutionException;

import org.opt4j.core.Objectives;
import org.opt4j.core.Objective.Sign;
import org.opt4j.core.problem.Evaluator;

import com.google.inject.Inject;

import io.vertx.core.Vertx;
import io.vertx.core.json.JsonObject;
import io.vertx.ext.web.client.WebClient;
import io.vertx.ext.web.codec.BodyCodec;

public class ConfigEvaluator implements Evaluator<Configuration> {

    private final Vertx vertx;
    private final WebClient client;

    @Inject
    public ConfigEvaluator() {
        this.vertx = Vertx.vertx();
        this.client = WebClient.create(this.vertx);
    }

    @Override
    public Objectives evaluate(Configuration phenotype) {

        CompletableFuture<Objectives> future = new CompletableFuture<>();
        JsonObject payload = intoPayload(phenotype);
        this.client.post(3000, "localhost", "/evaluate").as(BodyCodec.jsonObject()).sendJsonObject(payload)
                .onSuccess(res -> {
                    JsonObject response = res.body();
                    System.out.println("got response: " + response);
                    int cost = response.getInteger("cost");
                    float runtime = response.getFloat("runtime_s");

                    Objectives obj = new Objectives();
                    obj.add("cost", Sign.MIN, cost);
                    obj.add("runtime", Sign.MIN, runtime);

                    future.complete(obj);
                }).onFailure(res -> {
                    future.completeExceptionally(res.getCause());
                });

        try {
            return future.get();
        } catch (InterruptedException e) {
            throw new RuntimeException("interrupted while waiting for the server response: " + e);
        } catch (ExecutionException e) {
            throw new RuntimeException("exception while waiting for server response: " + e);
        }
    }

    private JsonObject intoPayload(Configuration config) {
        int coreNum = switch (config.getCoreNumber()) {
            case _2 -> 2;
            case _4 -> 4;
            case _8 -> 8;
            case _16 -> 16;
        };

        int memory = switch (config.getMemorySize()) {
            case _4_GB -> 4;
            case _8_GB -> 8;
            case _16_GB -> 16;
            case _32_GB -> 32;
            case _64_GB -> 64;
        };

        return new JsonObject().put("memory_size_gb", memory).put("core_number", coreNum)
                .put("disk_type", config.getDiskType());
    }
}
