FOR each phase
  FOR each failure
    FOR each load
      FOR each dag size
         RUN deterministic execution
         INJECT failure
         VERIFY:
            - no panic
            - CI truth respected
            - deterministic output
            - no resource overflow
