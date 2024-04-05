/**
 * Runs the given callback if `--allow-env` has not been set or
 * if the `TEST_MODE` env var is not `"true"`.
 */
export async function ifNotTesting(cb: () => void): Promise<void> {
	// See if we're even allowed to get env vars
	const { state } = await Deno.permissions.query({ name: "env" });
	const canGetEnv = state === "granted";

	if (!canGetEnv) {
		// Assuming production mode.
		cb();
		return;
	}

	if (Deno.env.get("TEST_MODE") === "true") {
		// Test mode
		return;
	}

	// Production mode
	cb();
}
