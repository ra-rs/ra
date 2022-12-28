#[doc = "Register `RDRHL_MAN` reader"]
pub struct R(crate::R<RDRHL_MAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDRHL_MAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDRHL_MAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDRHL_MAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDAT` reader - Serial receive data"]
pub type RDAT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MPB` reader - Multi-processor bit"]
pub type MPB_R = crate::BitReader<MPB_A>;
#[doc = "Multi-processor bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPB_A {
    #[doc = "0: Data transmission cycles"]
    _0 = 0,
    #[doc = "1: ID transmission cycles"]
    _1 = 1,
}
impl From<MPB_A> for bool {
    #[inline(always)]
    fn from(variant: MPB_A) -> Self {
        variant as u8 != 0
    }
}
impl MPB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPB_A {
        match self.bits {
            false => MPB_A::_0,
            true => MPB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MPB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MPB_A::_1
    }
}
#[doc = "Field `RSYNC` reader - Receive SYNC data bit"]
pub type RSYNC_R = crate::BitReader<RSYNC_A>;
#[doc = "Receive SYNC data bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSYNC_A {
    #[doc = "0: The received the Start Bit is DATA SYNC"]
    _0 = 0,
    #[doc = "1: The received the Start Bit is COMMAND SYNC"]
    _1 = 1,
}
impl From<RSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: RSYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl RSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSYNC_A {
        match self.bits {
            false => RSYNC_A::_0,
            true => RSYNC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSYNC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSYNC_A::_1
    }
}
impl R {
    #[doc = "Bits 0:8 - Serial receive data"]
    #[inline(always)]
    pub fn rdat(&self) -> RDAT_R {
        RDAT_R::new(self.bits & 0x01ff)
    }
    #[doc = "Bit 9 - Multi-processor bit"]
    #[inline(always)]
    pub fn mpb(&self) -> MPB_R {
        MPB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Receive SYNC data bit"]
    #[inline(always)]
    pub fn rsync(&self) -> RSYNC_R {
        RSYNC_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "Receive Data Register for Manchester mode (MMR.MANEN = 1)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdrhl_man](index.html) module"]
pub struct RDRHL_MAN_SPEC;
impl crate::RegisterSpec for RDRHL_MAN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rdrhl_man::R](R) reader structure"]
impl crate::Readable for RDRHL_MAN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RDRHL_MAN to value 0"]
impl crate::Resettable for RDRHL_MAN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
