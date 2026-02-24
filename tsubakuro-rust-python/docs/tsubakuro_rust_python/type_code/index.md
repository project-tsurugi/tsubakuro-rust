Module tsubakuro_rust_python.type_code
======================================

Classes
-------

`Bool(value=None)`
:   BOOLEAN type.
    
    Attributes:
        value (Optional[bool]): boolean value. (read only)

    ### Instance variables

    `value`
    :   Value.

`Bytes(value=None)`
:   BINARY, VARBINARY type.
    
    Attributes:
        value (Optional[bytes]): binary data. (read only)

    ### Instance variables

    `value`
    :   Value.

`Date(value=None)`
:   DATE type.
    
    Attributes:
        value (Optional[datetime.date]): date value. (read only)

    ### Instance variables

    `value`
    :   Value.

    ### Methods

    `of(year, month, day)`
    :   Create a `Date` from year, month, and day.
        
        Args:
            year (int): year
            month (int): month (1-12)
            day (int): day (1-31)
        
        Returns:
            Date: created `Date` object

    `raw(epoch_days)`
    :   Create a `Date` from epoch days.
        
        Args:
            epoch_days (int): number of days offset of epoch 1970-01-01
        
        Returns:
            Date: created `Date` object

`Datetime(value=None, nanosecond=None)`
:   TIMESTAMP type.
    
    Attributes:
        value (Optional[datetime.datetime]): datetime value. (read only)
        nanosecond (Optional[int]): nanosecond part of the time. (read only)

    ### Instance variables

    `nanosecond`
    :   Nnanosecond.

    `value`
    :   Value.

    ### Methods

    `of(year, month, day, hour=0, minute=0, second=0, nanosecond=0)`
    :   Create a `Datetime` from year, month, day, hour, minute, second, and nanosecond.
        
        Args:
            year (int): year
            month (int): month (1-12)
            day (int): day (1-31)
            hour (int, optional): hour (0-23)
            minute (int, optional): minute (0-59)
            second (int, optional): second (0-59)
            nanosecond (int, optional): nanosecond (0-999,999,999)
        
        Returns:
            Datetime: created `Datetime` object

    `raw(epoch_seconds, nanos)`
    :   Create a `Datetime` from epoch seconds and nanoseconds.
        
        Args:
            epoch_seconds (int): offset seconds from epoch (1970-01-01 00:00:00)
            nanos (int): nanosecond part of the time (0-999,999,999)
        
        Returns:
            Datetime: created `Datetime` object

`Decimal(value=None)`
:   DECIMAL type.
    
    Attributes:
        value (Optional[decimal.Decimal]): decimal value. (read only)

    ### Instance variables

    `value`
    :   Value.

    ### Methods

    `raw(unscaled_value, exponent)`
    :   Create a `Decimal` from unscaled value and exponent.
        
        Args:
            unscaled_value (bytes): unscaled value as big-endian byte array
            exponent (int): exponent
        
        Returns:
            Decimal: created `Decimal` object

`Float32(value=None)`
:   REAL type.
    
    Attributes:
        value (Optional[float]): float value. (read only)

    ### Instance variables

    `value`
    :   Value.

`Float64(value=None)`
:   DOUBLE type.
    
    Attributes:
        value (Optional[float]): float value. (read only)

    ### Instance variables

    `value`
    :   Value.

`Int32(value=None)`
:   INT type.
    
    Attributes:
        value (Optional[int]): integer value. (read only)

    ### Instance variables

    `value`
    :   Value.

`Int64(value=None)`
:   BIGINT type.
    
    Attributes:
        value (Optional[int]): integer value. (read only)

    ### Instance variables

    `value`
    :   Value.

`OffsetDatetime(value=None, nanosecond=None)`
:   TIMESTAMP WITH TIME ZONE type.
    
    Attributes:
        value (Optional[datetime.datetime]): datetime value with time zone. (read only)
        nanosecond (Optional[int]): nanosecond part of the time. (read only)

    ### Instance variables

    `nanosecond`
    :   Nnanosecond.

    `value`
    :   Value.

    ### Methods

    `of(year, month, day, hour=0, minute=0, second=0, nanosecond=0, tzinfo=None)`
    :   Create a `OffsetDatetime` from year, month, day, hour, minute, second, nanosecond, and tzinfo.
        
        Args:
            year (int): year
            month (int): month (1-12)
            day (int): day (1-31)
            hour (int, optional): hour (0-23)
            minute (int, optional): minute (0-59)
            second (int, optional): second (0-59)
            nanosecond (int, optional): nanosecond (0-999,999,999)
            tzinfo (datetime.tzinfo, optional): time zone info (default: UTC)
        
        Returns:
            OffsetDatetime: created `OffsetDatetime` object

    `raw(epoch_seconds, nanos, time_zone_offset)`
    :   Create a `OffsetDatetime` from epoch seconds, nanoseconds, and time zone offset.
        
        Args:
            epoch_seconds (int): offset seconds from epoch (1970-01-01 00:00:00)
            nanos (int): nanosecond part of the time (0-999,999,999)
            time_zone_offset (int): time zone offset in minutes
        
        Returns:
            OffsetDatetime: created `OffsetDatetime` object

`OffsetTime(value=None, nanosecond=None)`
:   TIME WITH TIME ZONE type.
    
    Attributes:
        value (Optional[datetime.time]): time value with time zone. (read only)
        nanosecond (Optional[int]): nanosecond part of the time. (read only)

    ### Instance variables

    `nanosecond`
    :   Nnanosecond.

    `value`
    :   Value.

    ### Methods

    `of(hour=0, minute=0, second=0, nanosecond=0, tzinfo=None)`
    :   Create a `OffsetTime` from hour, minute, second, nanosecond, and tzinfo.
        
        Args:
            hour (int, optional): hour (0-23)
            minute (int, optional): minute (0-59)
            second (int, optional): second (0-59)
            nanosecond (int, optional): nanosecond (0-999,999,999)
            tzinfo (datetime.tzinfo, optional): time zone info (Default: UTC)
        
        Returns:
            OffsetTime: created `OffsetTime` object

    `raw(nanoseconds_of_day, time_zone_offset)`
    :   Create a `OffsetTime` from epoch nanoseconds of day and time zone offset.
        
        Args:
            nanoseconds_of_day (int): offset nano-seconds from epoch (00:00:00) in the time zone
            time_zone_offset (int): timezone offset in minute
        
        Returns:
            OffsetTime: created `OffsetTime` object

`Str(value=None)`
:   CHAR, VARCHAR type.
    
    Attributes:
        value (Optional[str]): string value. (read only)

    ### Instance variables

    `value`
    :   Value.

`Time(value=None, nanosecond=None)`
:   TIME type.
    
    Attributes:
        value (Optional[datetime.time]): time value. (read only)
        nanosecond (Optional[int]): nanosecond part of the time. (read only)

    ### Instance variables

    `nanosecond`
    :   Nnanosecond.

    `value`
    :   Value.

    ### Methods

    `of(hour=0, minute=0, second=0, nanosecond=0)`
    :   Create a `Time` from hour, minute, second, and nanosecond.
        
        Args:
            hour (int, optional): hour (0-23)
            minute (int, optional): minute (0-59)
            second (int, optional): second (0-59)
            nanosecond (int, optional): nanosecond (0-999,999,999)
        
        Returns:
            Time: created `Time` object

    `raw(nanoseconds_of_day)`
    :   Create a `Time` from nanoseconds of day.
        
        Args:
            nanoseconds_of_day (int): time of day (nanoseconds since 00:00:00)
        
        Returns:
            Time: created `Time` object