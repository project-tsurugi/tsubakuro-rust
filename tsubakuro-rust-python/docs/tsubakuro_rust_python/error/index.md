Module tsubakuro_rust_python.error
==================================

Classes
-------

`AnalyzeException(*args, **kwargs)`
:   Tsurugi AnalyzeException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.CompileException
    * tsubakuro_rust_python.error.ProgrammingError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

    ### Descendants

    * tsubakuro_rust_python.error.SymbolAnalyzeException
    * tsubakuro_rust_python.error.TypeAnalyzeException
    * tsubakuro_rust_python.error.ValueAnalyzeException

`BlockedByConcurrentOperationException(*args, **kwargs)`
:   Tsurugi BlockedByConcurrentOperationException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.CcException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`BlockedByHighPriorityTransactionException(*args, **kwargs)`
:   Tsurugi BlockedByHighPriorityTransactionException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.SqlExecutionException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`CcException(*args, **kwargs)`
:   Tsurugi CcException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

    ### Descendants

    * tsubakuro_rust_python.error.BlockedByConcurrentOperationException
    * tsubakuro_rust_python.error.LtxException
    * tsubakuro_rust_python.error.OccException
    * tsubakuro_rust_python.error.RtxException

`CheckConstraintViolationException(*args, **kwargs)`
:   Tsurugi CheckConstraintViolationException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.ConstraintViolationException
    * tsubakuro_rust_python.error.IntegrityError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`CompileException(*args, **kwargs)`
:   Tsurugi CompileException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.ProgrammingError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

    ### Descendants

    * tsubakuro_rust_python.error.AnalyzeException
    * tsubakuro_rust_python.error.SyntaxException

`ConflictOnWritePreserveException(*args, **kwargs)`
:   Tsurugi ConflictOnWritePreserveException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.OccReadException
    * tsubakuro_rust_python.error.OccException
    * tsubakuro_rust_python.error.CcException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`ConstraintViolationException(*args, **kwargs)`
:   Tsurugi ConstraintViolationException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.IntegrityError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

    ### Descendants

    * tsubakuro_rust_python.error.CheckConstraintViolationException
    * tsubakuro_rust_python.error.NotNullConstraintViolationException
    * tsubakuro_rust_python.error.ReferentialIntegrityConstraintViolationException
    * tsubakuro_rust_python.error.UniqueConstraintViolationException

`DataCorruptionException(*args, **kwargs)`
:   Tsurugi DataCorruptionException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.SqlExecutionException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

    ### Descendants

    * tsubakuro_rust_python.error.SecondaryIndexCorruptionException

`DataError(*args, **kwargs)`
:   data error (PEP 249)

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

    ### Descendants

    * tsubakuro_rust_python.error.EvaluationException
    * tsubakuro_rust_python.error.SqlLimitReachedException

`DatabaseError(*args, **kwargs)`
:   database error (PEP 249)

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

    ### Descendants

    * tsubakuro_rust_python.error.DataError
    * tsubakuro_rust_python.error.IntegrityError
    * tsubakuro_rust_python.error.InternalError
    * tsubakuro_rust_python.error.NotSupportedError
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.ProgrammingError

`DependenciesViolationException(*args, **kwargs)`
:   Tsurugi DependenciesViolationException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.RestrictedOperationException
    * tsubakuro_rust_python.error.SqlExecutionException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`DumpDirectoryInaccessibleException(*args, **kwargs)`
:   Tsurugi DumpDirectoryInaccessibleException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.DumpFileException
    * tsubakuro_rust_python.error.SqlExecutionException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`DumpFileException(*args, **kwargs)`
:   Tsurugi DumpFileException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.SqlExecutionException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

    ### Descendants

    * tsubakuro_rust_python.error.DumpDirectoryInaccessibleException

`Error(*args, **kwargs)`
:   base class of all other exceptions (PEP 249)

    ### Ancestors (in MRO)

    * builtins.Exception
    * builtins.BaseException

    ### Descendants

    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.InterfaceError

`EvaluationException(*args, **kwargs)`
:   Tsurugi EvaluationException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.DataError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

    ### Descendants

    * tsubakuro_rust_python.error.ScalarSubqueryEvaluationException
    * tsubakuro_rust_python.error.ValueEvaluationException

`InactiveTransactionException(*args, **kwargs)`
:   Tsurugi InactiveTransactionException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.RestrictedOperationException
    * tsubakuro_rust_python.error.SqlExecutionException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`InconsistentStatementException(*args, **kwargs)`
:   Tsurugi InconsistentStatementException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.SqlExecutionException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`IntegrityError(*args, **kwargs)`
:   integrity error (PEP 249)

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

    ### Descendants

    * tsubakuro_rust_python.error.ConstraintViolationException

`InterfaceError(*args, **kwargs)`
:   interface error (PEP 249)

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`InternalError(*args, **kwargs)`
:   internal error (PEP 249)

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

    ### Descendants

    * tsubakuro_rust_python.error.InternalException

`InternalException(*args, **kwargs)`
:   Tsurugi InternalException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.InternalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`InvalidDecimalValueException(*args, **kwargs)`
:   Tsurugi InvalidDecimalValueException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.InvalidRuntimeValueException
    * tsubakuro_rust_python.error.SqlLimitReachedException
    * tsubakuro_rust_python.error.DataError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`InvalidRuntimeValueException(*args, **kwargs)`
:   Tsurugi InvalidRuntimeValueException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.SqlLimitReachedException
    * tsubakuro_rust_python.error.DataError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

    ### Descendants

    * tsubakuro_rust_python.error.InvalidDecimalValueException
    * tsubakuro_rust_python.error.ValueOutOfRangeException
    * tsubakuro_rust_python.error.ValueTooLongException

`LoadFileException(*args, **kwargs)`
:   Tsurugi LoadFileException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.SqlExecutionException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

    ### Descendants

    * tsubakuro_rust_python.error.LoadFileFormatException
    * tsubakuro_rust_python.error.LoadFileNotFoundException

`LoadFileFormatException(*args, **kwargs)`
:   Tsurugi LoadFileFormatException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.LoadFileException
    * tsubakuro_rust_python.error.SqlExecutionException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`LoadFileNotFoundException(*args, **kwargs)`
:   Tsurugi LoadFileNotFoundException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.LoadFileException
    * tsubakuro_rust_python.error.SqlExecutionException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`LtxException(*args, **kwargs)`
:   Tsurugi LtxException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.CcException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

    ### Descendants

    * tsubakuro_rust_python.error.LtxReadException
    * tsubakuro_rust_python.error.LtxWriteException

`LtxReadException(*args, **kwargs)`
:   Tsurugi LtxReadException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.LtxException
    * tsubakuro_rust_python.error.CcException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`LtxWriteException(*args, **kwargs)`
:   Tsurugi LtxWriteException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.LtxException
    * tsubakuro_rust_python.error.CcException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`LtxWriteOperationWithoutWritePreserveException(*args, **kwargs)`
:   Tsurugi LtxWriteOperationWithoutWritePreserveException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.RestrictedOperationException
    * tsubakuro_rust_python.error.SqlExecutionException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`NotNullConstraintViolationException(*args, **kwargs)`
:   Tsurugi NotNullConstraintViolationException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.ConstraintViolationException
    * tsubakuro_rust_python.error.IntegrityError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`NotSupportedError(*args, **kwargs)`
:   not supported error (PEP 249)

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

    ### Descendants

    * tsubakuro_rust_python.error.UnsupportedCompilerFeatureException
    * tsubakuro_rust_python.error.UnsupportedRuntimeFeatureException

`OccException(*args, **kwargs)`
:   Tsurugi OccException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.CcException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

    ### Descendants

    * tsubakuro_rust_python.error.OccReadException
    * tsubakuro_rust_python.error.OccWriteException

`OccReadException(*args, **kwargs)`
:   Tsurugi OccReadException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.OccException
    * tsubakuro_rust_python.error.CcException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

    ### Descendants

    * tsubakuro_rust_python.error.ConflictOnWritePreserveException

`OccWriteException(*args, **kwargs)`
:   Tsurugi OccWriteException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.OccException
    * tsubakuro_rust_python.error.CcException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`OperationalError(*args, **kwargs)`
:   operation error (PEP 249)

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

    ### Descendants

    * tsubakuro_rust_python.error.ServerException

`ParameterException(*args, **kwargs)`
:   Tsurugi ParameterException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.ProgrammingError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

    ### Descendants

    * tsubakuro_rust_python.error.UnresolvedPlaceholderException

`ProgrammingError(*args, **kwargs)`
:   programming error (PEP 249)

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

    ### Descendants

    * tsubakuro_rust_python.error.CompileException
    * tsubakuro_rust_python.error.ParameterException

`ReadOperationOnRestrictedReadAreaException(*args, **kwargs)`
:   Tsurugi ReadOperationOnRestrictedReadAreaException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.RestrictedOperationException
    * tsubakuro_rust_python.error.SqlExecutionException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`ReferentialIntegrityConstraintViolationException(*args, **kwargs)`
:   Tsurugi ReferentialIntegrityConstraintViolationException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.ConstraintViolationException
    * tsubakuro_rust_python.error.IntegrityError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`RequestFailureException(*args, **kwargs)`
:   Tsurugi RequestFailureException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.SqlExecutionException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

    ### Descendants

    * tsubakuro_rust_python.error.StatementNotFoundException
    * tsubakuro_rust_python.error.TransactionNotFoundException

`RestrictedOperationException(*args, **kwargs)`
:   Tsurugi RestrictedOperationException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.SqlExecutionException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

    ### Descendants

    * tsubakuro_rust_python.error.DependenciesViolationException
    * tsubakuro_rust_python.error.InactiveTransactionException
    * tsubakuro_rust_python.error.LtxWriteOperationWithoutWritePreserveException
    * tsubakuro_rust_python.error.ReadOperationOnRestrictedReadAreaException
    * tsubakuro_rust_python.error.WriteOperationByRtxException

`RtxException(*args, **kwargs)`
:   Tsurugi RtxException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.CcException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`ScalarSubqueryEvaluationException(*args, **kwargs)`
:   Tsurugi ScalarSubqueryEvaluationException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.EvaluationException
    * tsubakuro_rust_python.error.DataError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`SecondaryIndexCorruptionException(*args, **kwargs)`
:   Tsurugi SecondaryIndexCorruptionException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.DataCorruptionException
    * tsubakuro_rust_python.error.SqlExecutionException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`ServerException(*args, **kwargs)`
:   Tsurugi ServerException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

    ### Descendants

    * tsubakuro_rust_python.error.SqlServiceException

`SqlExecutionException(*args, **kwargs)`
:   Tsurugi SqlExecutionException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

    ### Descendants

    * tsubakuro_rust_python.error.BlockedByHighPriorityTransactionException
    * tsubakuro_rust_python.error.DataCorruptionException
    * tsubakuro_rust_python.error.DumpFileException
    * tsubakuro_rust_python.error.InconsistentStatementException
    * tsubakuro_rust_python.error.LoadFileException
    * tsubakuro_rust_python.error.RequestFailureException
    * tsubakuro_rust_python.error.RestrictedOperationException
    * tsubakuro_rust_python.error.SqlRequestTimeoutException
    * tsubakuro_rust_python.error.TargetAlreadyExistsException
    * tsubakuro_rust_python.error.TargetNotFoundException

`SqlLimitReachedException(*args, **kwargs)`
:   Tsurugi SqlLimitReachedException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.DataError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

    ### Descendants

    * tsubakuro_rust_python.error.InvalidRuntimeValueException
    * tsubakuro_rust_python.error.TransactionExceededLimitException

`SqlRequestTimeoutException(*args, **kwargs)`
:   Tsurugi SqlRequestTimeoutException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.SqlExecutionException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`SqlServiceException(*args, **kwargs)`
:   Tsurugi SqlServiceException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

    ### Descendants

    * tsubakuro_rust_python.error.CcException
    * tsubakuro_rust_python.error.SqlExecutionException

`StatementNotFoundException(*args, **kwargs)`
:   Tsurugi StatementNotFoundException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.RequestFailureException
    * tsubakuro_rust_python.error.SqlExecutionException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`SymbolAnalyzeException(*args, **kwargs)`
:   Tsurugi SymbolAnalyzeException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.AnalyzeException
    * tsubakuro_rust_python.error.CompileException
    * tsubakuro_rust_python.error.ProgrammingError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`SyntaxException(*args, **kwargs)`
:   Tsurugi SyntaxException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.CompileException
    * tsubakuro_rust_python.error.ProgrammingError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`TargetAlreadyExistsException(*args, **kwargs)`
:   Tsurugi TargetAlreadyExistsException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.SqlExecutionException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`TargetNotFoundException(*args, **kwargs)`
:   Tsurugi TargetNotFoundException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.SqlExecutionException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`TransactionExceededLimitException(*args, **kwargs)`
:   Tsurugi TransactionExceededLimitException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.SqlLimitReachedException
    * tsubakuro_rust_python.error.DataError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`TransactionNotFoundException(*args, **kwargs)`
:   Tsurugi TransactionNotFoundException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.RequestFailureException
    * tsubakuro_rust_python.error.SqlExecutionException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`TypeAnalyzeException(*args, **kwargs)`
:   Tsurugi TypeAnalyzeException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.AnalyzeException
    * tsubakuro_rust_python.error.CompileException
    * tsubakuro_rust_python.error.ProgrammingError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`UniqueConstraintViolationException(*args, **kwargs)`
:   Tsurugi UniqueConstraintViolationException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.ConstraintViolationException
    * tsubakuro_rust_python.error.IntegrityError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`UnresolvedPlaceholderException(*args, **kwargs)`
:   Tsurugi UnresolvedPlaceholderException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.ParameterException
    * tsubakuro_rust_python.error.ProgrammingError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`UnsupportedCompilerFeatureException(*args, **kwargs)`
:   Tsurugi UnsupportedCompilerFeatureException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.NotSupportedError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`UnsupportedRuntimeFeatureException(*args, **kwargs)`
:   Tsurugi UnsupportedRuntimeFeatureException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.NotSupportedError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`ValueAnalyzeException(*args, **kwargs)`
:   Tsurugi ValueAnalyzeException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.AnalyzeException
    * tsubakuro_rust_python.error.CompileException
    * tsubakuro_rust_python.error.ProgrammingError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`ValueEvaluationException(*args, **kwargs)`
:   Tsurugi ValueEvaluationException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.EvaluationException
    * tsubakuro_rust_python.error.DataError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`ValueOutOfRangeException(*args, **kwargs)`
:   Tsurugi ValueOutOfRangeException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.InvalidRuntimeValueException
    * tsubakuro_rust_python.error.SqlLimitReachedException
    * tsubakuro_rust_python.error.DataError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`ValueTooLongException(*args, **kwargs)`
:   Tsurugi ValueTooLongException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.InvalidRuntimeValueException
    * tsubakuro_rust_python.error.SqlLimitReachedException
    * tsubakuro_rust_python.error.DataError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException

`Warning(*args, **kwargs)`
:   important warning (PEP 249)

    ### Ancestors (in MRO)

    * builtins.Exception
    * builtins.BaseException

`WriteOperationByRtxException(*args, **kwargs)`
:   Tsurugi WriteOperationByRtxException

    ### Ancestors (in MRO)

    * tsubakuro_rust_python.error.RestrictedOperationException
    * tsubakuro_rust_python.error.SqlExecutionException
    * tsubakuro_rust_python.error.SqlServiceException
    * tsubakuro_rust_python.error.ServerException
    * tsubakuro_rust_python.error.OperationalError
    * tsubakuro_rust_python.error.DatabaseError
    * tsubakuro_rust_python.error.Error
    * builtins.Exception
    * builtins.BaseException