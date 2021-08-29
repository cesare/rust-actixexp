-- CreateTable
CREATE TABLE "identities" (
    "id" UUID NOT NULL,
    "provider_identifier" VARCHAR(255) NOT NULL,
    "alive" BOOLEAN NOT NULL DEFAULT true,
    "registered_at" TIMESTAMPTZ(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "identities.provider_identifier_unique" ON "identities"("provider_identifier");
