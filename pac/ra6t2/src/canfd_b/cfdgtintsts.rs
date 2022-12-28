#[doc = "Register `CFDGTINTSTS` reader"]
pub struct R(crate::R<CFDGTINTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDGTINTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDGTINTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDGTINTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TSIF0` reader - TX Successful Interrupt Flag"]
pub type TSIF0_R = crate::BitReader<TSIF0_A>;
#[doc = "TX Successful Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSIF0_A {
    #[doc = "0: Channel n TX Successful Interrupt flag not set"]
    _0 = 0,
    #[doc = "1: Channel n TX Successful Interrupt flag set"]
    _1 = 1,
}
impl From<TSIF0_A> for bool {
    #[inline(always)]
    fn from(variant: TSIF0_A) -> Self {
        variant as u8 != 0
    }
}
impl TSIF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSIF0_A {
        match self.bits {
            false => TSIF0_A::_0,
            true => TSIF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSIF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSIF0_A::_1
    }
}
#[doc = "Field `TAI0` reader - TX Abort Interrupt Flag"]
pub type TAI0_R = crate::BitReader<TAI0_A>;
#[doc = "TX Abort Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAI0_A {
    #[doc = "0: Channel n TX Abort Interrupt flag not set"]
    _0 = 0,
    #[doc = "1: Channel n TX Abort Interrupt flag set"]
    _1 = 1,
}
impl From<TAI0_A> for bool {
    #[inline(always)]
    fn from(variant: TAI0_A) -> Self {
        variant as u8 != 0
    }
}
impl TAI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAI0_A {
        match self.bits {
            false => TAI0_A::_0,
            true => TAI0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TAI0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TAI0_A::_1
    }
}
#[doc = "Field `TQIF0` reader - TX Queue Interrupt Flag"]
pub type TQIF0_R = crate::BitReader<TQIF0_A>;
#[doc = "TX Queue Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TQIF0_A {
    #[doc = "0: Channel n TX Queue Interrupt flag not set"]
    _0 = 0,
    #[doc = "1: Channel n TX Queue Interrupt flag set"]
    _1 = 1,
}
impl From<TQIF0_A> for bool {
    #[inline(always)]
    fn from(variant: TQIF0_A) -> Self {
        variant as u8 != 0
    }
}
impl TQIF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TQIF0_A {
        match self.bits {
            false => TQIF0_A::_0,
            true => TQIF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TQIF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TQIF0_A::_1
    }
}
#[doc = "Field `CFTIF0` reader - COM FIFO TX Mode Interrupt Flag"]
pub type CFTIF0_R = crate::BitReader<CFTIF0_A>;
#[doc = "COM FIFO TX Mode Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFTIF0_A {
    #[doc = "0: Channel n COM FIFO TX Mode Interrupt flag not set"]
    _0 = 0,
    #[doc = "1: Channel n COM FIFO TX Mode Interrupt flag set"]
    _1 = 1,
}
impl From<CFTIF0_A> for bool {
    #[inline(always)]
    fn from(variant: CFTIF0_A) -> Self {
        variant as u8 != 0
    }
}
impl CFTIF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFTIF0_A {
        match self.bits {
            false => CFTIF0_A::_0,
            true => CFTIF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFTIF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFTIF0_A::_1
    }
}
#[doc = "Field `THIF0` reader - TX History List Interrupt"]
pub type THIF0_R = crate::BitReader<THIF0_A>;
#[doc = "TX History List Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum THIF0_A {
    #[doc = "0: Channel n TX History List Interrupt flag not set"]
    _0 = 0,
    #[doc = "1: Channel n TX History List Interrupt flag set"]
    _1 = 1,
}
impl From<THIF0_A> for bool {
    #[inline(always)]
    fn from(variant: THIF0_A) -> Self {
        variant as u8 != 0
    }
}
impl THIF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THIF0_A {
        match self.bits {
            false => THIF0_A::_0,
            true => THIF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == THIF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == THIF0_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - TX Successful Interrupt Flag"]
    #[inline(always)]
    pub fn tsif0(&self) -> TSIF0_R {
        TSIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Abort Interrupt Flag"]
    #[inline(always)]
    pub fn tai0(&self) -> TAI0_R {
        TAI0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX Queue Interrupt Flag"]
    #[inline(always)]
    pub fn tqif0(&self) -> TQIF0_R {
        TQIF0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - COM FIFO TX Mode Interrupt Flag"]
    #[inline(always)]
    pub fn cftif0(&self) -> CFTIF0_R {
        CFTIF0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX History List Interrupt"]
    #[inline(always)]
    pub fn thif0(&self) -> THIF0_R {
        THIF0_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Global TX Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdgtintsts](index.html) module"]
pub struct CFDGTINTSTS_SPEC;
impl crate::RegisterSpec for CFDGTINTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdgtintsts::R](R) reader structure"]
impl crate::Readable for CFDGTINTSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFDGTINTSTS to value 0"]
impl crate::Resettable for CFDGTINTSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
