package com.tsurugidb.tsubakuro.rust.java.session;

import org.junit.jupiter.api.Test;

import com.tsurugidb.tsubakuro.rust.java.context.TgFfiContext;
import com.tsurugidb.tsubakuro.rust.java.util.TgFfiTester;

class TgFfiSessionTest extends TgFfiTester {

	@Test
	void connect() {
		var manager = getFfiObjectManager();

		var context = TgFfiContext.create(manager);

		var connectionOption = TgFfiConnectionOption.create(context);
		connectionOption.setEndpointUrl(context, getEndpoint());

		try (var session = TgFfiSession.connect(context, connectionOption)) {
		}
	}
}
