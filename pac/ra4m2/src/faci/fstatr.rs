#[doc = "Register `FSTATR` reader"]
pub struct R(crate::R<FSTATR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSTATR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSTATR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSTATR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSTATR` writer"]
pub struct W(crate::W<FSTATR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSTATR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FSTATR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSTATR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLWEERR` reader - Flash Write/Erase Protect Error Flag"]
pub type FLWEERR_R = crate::BitReader<FLWEERR_A>;
#[doc = "Flash Write/Erase Protect Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLWEERR_A {
    #[doc = "0: An error has not occurred"]
    _0 = 0,
    #[doc = "1: An error has occurred."]
    _1 = 1,
}
impl From<FLWEERR_A> for bool {
    #[inline(always)]
    fn from(variant: FLWEERR_A) -> Self {
        variant as u8 != 0
    }
}
impl FLWEERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLWEERR_A {
        match self.bits {
            false => FLWEERR_A::_0,
            true => FLWEERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLWEERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLWEERR_A::_1
    }
}
#[doc = "Field `PRGSPD` reader - Programming Suspend Status Flag"]
pub type PRGSPD_R = crate::BitReader<PRGSPD_A>;
#[doc = "Programming Suspend Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRGSPD_A {
    #[doc = "0: The flash sequencer is in a state other than those corresponding to the value 1"]
    _0 = 0,
    #[doc = "1: The flash sequencer is in the programming suspension processing state or programming suspended state."]
    _1 = 1,
}
impl From<PRGSPD_A> for bool {
    #[inline(always)]
    fn from(variant: PRGSPD_A) -> Self {
        variant as u8 != 0
    }
}
impl PRGSPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRGSPD_A {
        match self.bits {
            false => PRGSPD_A::_0,
            true => PRGSPD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRGSPD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRGSPD_A::_1
    }
}
#[doc = "Field `ERSSPD` reader - Erasure Suspend Status Flag"]
pub type ERSSPD_R = crate::BitReader<ERSSPD_A>;
#[doc = "Erasure Suspend Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERSSPD_A {
    #[doc = "0: The flash sequencer is in a state other than those corresponding to the value 1"]
    _0 = 0,
    #[doc = "1: The flash sequencer is in the erasure suspension processing state or the erasure suspended state."]
    _1 = 1,
}
impl From<ERSSPD_A> for bool {
    #[inline(always)]
    fn from(variant: ERSSPD_A) -> Self {
        variant as u8 != 0
    }
}
impl ERSSPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERSSPD_A {
        match self.bits {
            false => ERSSPD_A::_0,
            true => ERSSPD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERSSPD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERSSPD_A::_1
    }
}
#[doc = "Field `DBFULL` reader - Data Buffer Full Flag"]
pub type DBFULL_R = crate::BitReader<DBFULL_A>;
#[doc = "Data Buffer Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBFULL_A {
    #[doc = "0: The data buffer is empty"]
    _0 = 0,
    #[doc = "1: The data buffer is full."]
    _1 = 1,
}
impl From<DBFULL_A> for bool {
    #[inline(always)]
    fn from(variant: DBFULL_A) -> Self {
        variant as u8 != 0
    }
}
impl DBFULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBFULL_A {
        match self.bits {
            false => DBFULL_A::_0,
            true => DBFULL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBFULL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBFULL_A::_1
    }
}
#[doc = "Field `SUSRDY` reader - Suspend Ready Flag"]
pub type SUSRDY_R = crate::BitReader<SUSRDY_A>;
#[doc = "Suspend Ready Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSRDY_A {
    #[doc = "0: The flash sequencer cannot receive P/E suspend commands"]
    _0 = 0,
    #[doc = "1: The flash sequencer can receive P/E suspend commands."]
    _1 = 1,
}
impl From<SUSRDY_A> for bool {
    #[inline(always)]
    fn from(variant: SUSRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl SUSRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUSRDY_A {
        match self.bits {
            false => SUSRDY_A::_0,
            true => SUSRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SUSRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SUSRDY_A::_1
    }
}
#[doc = "Field `PRGERR` reader - Programming Error Flag"]
pub type PRGERR_R = crate::BitReader<PRGERR_A>;
#[doc = "Programming Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRGERR_A {
    #[doc = "0: Programming has completed successfully"]
    _0 = 0,
    #[doc = "1: An error has occurred during programming."]
    _1 = 1,
}
impl From<PRGERR_A> for bool {
    #[inline(always)]
    fn from(variant: PRGERR_A) -> Self {
        variant as u8 != 0
    }
}
impl PRGERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRGERR_A {
        match self.bits {
            false => PRGERR_A::_0,
            true => PRGERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRGERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRGERR_A::_1
    }
}
#[doc = "Field `ERSERR` reader - Erasure Error Flag"]
pub type ERSERR_R = crate::BitReader<ERSERR_A>;
#[doc = "Erasure Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERSERR_A {
    #[doc = "0: Erasure has completed successfully"]
    _0 = 0,
    #[doc = "1: An error has occurred during erasure."]
    _1 = 1,
}
impl From<ERSERR_A> for bool {
    #[inline(always)]
    fn from(variant: ERSERR_A) -> Self {
        variant as u8 != 0
    }
}
impl ERSERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERSERR_A {
        match self.bits {
            false => ERSERR_A::_0,
            true => ERSERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERSERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERSERR_A::_1
    }
}
#[doc = "Field `ILGLERR` reader - Illegal Command Error Flag"]
pub type ILGLERR_R = crate::BitReader<ILGLERR_A>;
#[doc = "Illegal Command Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILGLERR_A {
    #[doc = "0: The flash sequencer has not detected an illegal FACI command or illegal flash memory access"]
    _0 = 0,
    #[doc = "1: The flash sequencer has detected an illegal FACI command or illegal flash memory access."]
    _1 = 1,
}
impl From<ILGLERR_A> for bool {
    #[inline(always)]
    fn from(variant: ILGLERR_A) -> Self {
        variant as u8 != 0
    }
}
impl ILGLERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILGLERR_A {
        match self.bits {
            false => ILGLERR_A::_0,
            true => ILGLERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILGLERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILGLERR_A::_1
    }
}
#[doc = "Field `FRDY` reader - Flash Ready Flag"]
pub type FRDY_R = crate::BitReader<FRDY_A>;
#[doc = "Flash Ready Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRDY_A {
    #[doc = "0: Program, Block Erase, Multi Block Erase, P/E suspend, P/E resume, Forced Stop, Blank Check, or Configuration set command processing is in progress"]
    _0 = 0,
    #[doc = "1: None of the above is in progress."]
    _1 = 1,
}
impl From<FRDY_A> for bool {
    #[inline(always)]
    fn from(variant: FRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl FRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRDY_A {
        match self.bits {
            false => FRDY_A::_0,
            true => FRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRDY_A::_1
    }
}
#[doc = "Field `OTERR` reader - Other Error"]
pub type OTERR_R = crate::BitReader<OTERR_A>;
#[doc = "Other Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTERR_A {
    #[doc = "0: A status clear or forced stop command processing is complete"]
    _0 = 0,
    #[doc = "1: An error has occurred."]
    _1 = 1,
}
impl From<OTERR_A> for bool {
    #[inline(always)]
    fn from(variant: OTERR_A) -> Self {
        variant as u8 != 0
    }
}
impl OTERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OTERR_A {
        match self.bits {
            false => OTERR_A::_0,
            true => OTERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OTERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OTERR_A::_1
    }
}
#[doc = "Field `SECERR` reader - Security Error"]
pub type SECERR_R = crate::BitReader<SECERR_A>;
#[doc = "Security Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECERR_A {
    #[doc = "0: A status clear or forced stop command processing is complete"]
    _0 = 0,
    #[doc = "1: An error has occurred."]
    _1 = 1,
}
impl From<SECERR_A> for bool {
    #[inline(always)]
    fn from(variant: SECERR_A) -> Self {
        variant as u8 != 0
    }
}
impl SECERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECERR_A {
        match self.bits {
            false => SECERR_A::_0,
            true => SECERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SECERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SECERR_A::_1
    }
}
#[doc = "Field `SECERR` writer - Security Error"]
pub type SECERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSTATR_SPEC, SECERR_A, O>;
impl<'a, const O: u8> SECERR_W<'a, O> {
    #[doc = "A status clear or forced stop command processing is complete"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SECERR_A::_0)
    }
    #[doc = "An error has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SECERR_A::_1)
    }
}
#[doc = "Field `FESETERR` reader - FENTRY Setting Error"]
pub type FESETERR_R = crate::BitReader<FESETERR_A>;
#[doc = "FENTRY Setting Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FESETERR_A {
    #[doc = "0: A status clear or forced stop command processing is complete"]
    _0 = 0,
    #[doc = "1: An error has occurred."]
    _1 = 1,
}
impl From<FESETERR_A> for bool {
    #[inline(always)]
    fn from(variant: FESETERR_A) -> Self {
        variant as u8 != 0
    }
}
impl FESETERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FESETERR_A {
        match self.bits {
            false => FESETERR_A::_0,
            true => FESETERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FESETERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FESETERR_A::_1
    }
}
#[doc = "Field `ILGCOMERR` reader - Illegal Command Error"]
pub type ILGCOMERR_R = crate::BitReader<ILGCOMERR_A>;
#[doc = "Illegal Command Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILGCOMERR_A {
    #[doc = "0: A status clear or forced stop command processing is complete"]
    _0 = 0,
    #[doc = "1: An error has occurred."]
    _1 = 1,
}
impl From<ILGCOMERR_A> for bool {
    #[inline(always)]
    fn from(variant: ILGCOMERR_A) -> Self {
        variant as u8 != 0
    }
}
impl ILGCOMERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILGCOMERR_A {
        match self.bits {
            false => ILGCOMERR_A::_0,
            true => ILGCOMERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILGCOMERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILGCOMERR_A::_1
    }
}
impl R {
    #[doc = "Bit 6 - Flash Write/Erase Protect Error Flag"]
    #[inline(always)]
    pub fn flweerr(&self) -> FLWEERR_R {
        FLWEERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Programming Suspend Status Flag"]
    #[inline(always)]
    pub fn prgspd(&self) -> PRGSPD_R {
        PRGSPD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Erasure Suspend Status Flag"]
    #[inline(always)]
    pub fn ersspd(&self) -> ERSSPD_R {
        ERSSPD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data Buffer Full Flag"]
    #[inline(always)]
    pub fn dbfull(&self) -> DBFULL_R {
        DBFULL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Suspend Ready Flag"]
    #[inline(always)]
    pub fn susrdy(&self) -> SUSRDY_R {
        SUSRDY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Programming Error Flag"]
    #[inline(always)]
    pub fn prgerr(&self) -> PRGERR_R {
        PRGERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Erasure Error Flag"]
    #[inline(always)]
    pub fn erserr(&self) -> ERSERR_R {
        ERSERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Illegal Command Error Flag"]
    #[inline(always)]
    pub fn ilglerr(&self) -> ILGLERR_R {
        ILGLERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Flash Ready Flag"]
    #[inline(always)]
    pub fn frdy(&self) -> FRDY_R {
        FRDY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - Other Error"]
    #[inline(always)]
    pub fn oterr(&self) -> OTERR_R {
        OTERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Security Error"]
    #[inline(always)]
    pub fn secerr(&self) -> SECERR_R {
        SECERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - FENTRY Setting Error"]
    #[inline(always)]
    pub fn feseterr(&self) -> FESETERR_R {
        FESETERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Illegal Command Error"]
    #[inline(always)]
    pub fn ilgcomerr(&self) -> ILGCOMERR_R {
        ILGCOMERR_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - Security Error"]
    #[inline(always)]
    #[must_use]
    pub fn secerr(&mut self) -> SECERR_W<21> {
        SECERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fstatr](index.html) module"]
pub struct FSTATR_SPEC;
impl crate::RegisterSpec for FSTATR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fstatr::R](R) reader structure"]
impl crate::Readable for FSTATR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fstatr::W](W) writer structure"]
impl crate::Writable for FSTATR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSTATR to value 0x8000"]
impl crate::Resettable for FSTATR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
