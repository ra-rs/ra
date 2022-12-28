#[doc = "Register `PSARC` reader"]
pub struct R(crate::R<PSARC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSARC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSARC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSARC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSARC` writer"]
pub struct W(crate::W<PSARC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSARC_SPEC>;
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
impl From<crate::W<PSARC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSARC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSARC0` reader - CAC and the MSTPCRC.MSTPC0 bit security attribution"]
pub type PSARC0_R = crate::BitReader<PSARC0_A>;
#[doc = "CAC and the MSTPCRC.MSTPC0 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARC0_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARC0_A> for bool {
    #[inline(always)]
    fn from(variant: PSARC0_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARC0_A {
        match self.bits {
            false => PSARC0_A::_0,
            true => PSARC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARC0_A::_1
    }
}
#[doc = "Field `PSARC0` writer - CAC and the MSTPCRC.MSTPC0 bit security attribution"]
pub type PSARC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARC_SPEC, PSARC0_A, O>;
impl<'a, const O: u8> PSARC0_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARC0_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARC0_A::_1)
    }
}
#[doc = "Field `PSARC1` reader - CRC and the MSTPCRC.MSTPC1 bit security attribution"]
pub type PSARC1_R = crate::BitReader<PSARC1_A>;
#[doc = "CRC and the MSTPCRC.MSTPC1 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARC1_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARC1_A> for bool {
    #[inline(always)]
    fn from(variant: PSARC1_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARC1_A {
        match self.bits {
            false => PSARC1_A::_0,
            true => PSARC1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARC1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARC1_A::_1
    }
}
#[doc = "Field `PSARC1` writer - CRC and the MSTPCRC.MSTPC1 bit security attribution"]
pub type PSARC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARC_SPEC, PSARC1_A, O>;
impl<'a, const O: u8> PSARC1_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARC1_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARC1_A::_1)
    }
}
#[doc = "Field `PSARC13` reader - DOC and the MSTPCRC.MSTPC13 bit security attribution"]
pub type PSARC13_R = crate::BitReader<PSARC13_A>;
#[doc = "DOC and the MSTPCRC.MSTPC13 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARC13_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARC13_A> for bool {
    #[inline(always)]
    fn from(variant: PSARC13_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARC13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARC13_A {
        match self.bits {
            false => PSARC13_A::_0,
            true => PSARC13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARC13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARC13_A::_1
    }
}
#[doc = "Field `PSARC13` writer - DOC and the MSTPCRC.MSTPC13 bit security attribution"]
pub type PSARC13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARC_SPEC, PSARC13_A, O>;
impl<'a, const O: u8> PSARC13_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARC13_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARC13_A::_1)
    }
}
#[doc = "Field `PSARC20` reader - TFU and the MSTPCRC.MSTPC20 bit security attribution"]
pub type PSARC20_R = crate::BitReader<PSARC20_A>;
#[doc = "TFU and the MSTPCRC.MSTPC20 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARC20_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARC20_A> for bool {
    #[inline(always)]
    fn from(variant: PSARC20_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARC20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARC20_A {
        match self.bits {
            false => PSARC20_A::_0,
            true => PSARC20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARC20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARC20_A::_1
    }
}
#[doc = "Field `PSARC20` writer - TFU and the MSTPCRC.MSTPC20 bit security attribution"]
pub type PSARC20_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARC_SPEC, PSARC20_A, O>;
impl<'a, const O: u8> PSARC20_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARC20_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARC20_A::_1)
    }
}
#[doc = "Field `PSARC21` reader - IIRFA and the MSTPCRC.MSTPC21 bit security attribution"]
pub type PSARC21_R = crate::BitReader<PSARC21_A>;
#[doc = "IIRFA and the MSTPCRC.MSTPC21 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARC21_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARC21_A> for bool {
    #[inline(always)]
    fn from(variant: PSARC21_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARC21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARC21_A {
        match self.bits {
            false => PSARC21_A::_0,
            true => PSARC21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARC21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARC21_A::_1
    }
}
#[doc = "Field `PSARC21` writer - IIRFA and the MSTPCRC.MSTPC21 bit security attribution"]
pub type PSARC21_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARC_SPEC, PSARC21_A, O>;
impl<'a, const O: u8> PSARC21_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARC21_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARC21_A::_1)
    }
}
#[doc = "Field `PSARC27` reader - CANFD and the MSTPCRC.MSTPC27 bit security attribution"]
pub type PSARC27_R = crate::BitReader<PSARC27_A>;
#[doc = "CANFD and the MSTPCRC.MSTPC27 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARC27_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARC27_A> for bool {
    #[inline(always)]
    fn from(variant: PSARC27_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARC27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARC27_A {
        match self.bits {
            false => PSARC27_A::_0,
            true => PSARC27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARC27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARC27_A::_1
    }
}
#[doc = "Field `PSARC27` writer - CANFD and the MSTPCRC.MSTPC27 bit security attribution"]
pub type PSARC27_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARC_SPEC, PSARC27_A, O>;
impl<'a, const O: u8> PSARC27_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARC27_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARC27_A::_1)
    }
}
#[doc = "Field `PSARC31` reader - SCE5 and the MSTPCRC.MSTPC31 bit security attribution"]
pub type PSARC31_R = crate::BitReader<PSARC31_A>;
#[doc = "SCE5 and the MSTPCRC.MSTPC31 bit security attribution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSARC31_A {
    #[doc = "0: Secure"]
    _0 = 0,
    #[doc = "1: Non-secure"]
    _1 = 1,
}
impl From<PSARC31_A> for bool {
    #[inline(always)]
    fn from(variant: PSARC31_A) -> Self {
        variant as u8 != 0
    }
}
impl PSARC31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSARC31_A {
        match self.bits {
            false => PSARC31_A::_0,
            true => PSARC31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSARC31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSARC31_A::_1
    }
}
#[doc = "Field `PSARC31` writer - SCE5 and the MSTPCRC.MSTPC31 bit security attribution"]
pub type PSARC31_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSARC_SPEC, PSARC31_A, O>;
impl<'a, const O: u8> PSARC31_W<'a, O> {
    #[doc = "Secure"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSARC31_A::_0)
    }
    #[doc = "Non-secure"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSARC31_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - CAC and the MSTPCRC.MSTPC0 bit security attribution"]
    #[inline(always)]
    pub fn psarc0(&self) -> PSARC0_R {
        PSARC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CRC and the MSTPCRC.MSTPC1 bit security attribution"]
    #[inline(always)]
    pub fn psarc1(&self) -> PSARC1_R {
        PSARC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 13 - DOC and the MSTPCRC.MSTPC13 bit security attribution"]
    #[inline(always)]
    pub fn psarc13(&self) -> PSARC13_R {
        PSARC13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 20 - TFU and the MSTPCRC.MSTPC20 bit security attribution"]
    #[inline(always)]
    pub fn psarc20(&self) -> PSARC20_R {
        PSARC20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - IIRFA and the MSTPCRC.MSTPC21 bit security attribution"]
    #[inline(always)]
    pub fn psarc21(&self) -> PSARC21_R {
        PSARC21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 27 - CANFD and the MSTPCRC.MSTPC27 bit security attribution"]
    #[inline(always)]
    pub fn psarc27(&self) -> PSARC27_R {
        PSARC27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - SCE5 and the MSTPCRC.MSTPC31 bit security attribution"]
    #[inline(always)]
    pub fn psarc31(&self) -> PSARC31_R {
        PSARC31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAC and the MSTPCRC.MSTPC0 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarc0(&mut self) -> PSARC0_W<0> {
        PSARC0_W::new(self)
    }
    #[doc = "Bit 1 - CRC and the MSTPCRC.MSTPC1 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarc1(&mut self) -> PSARC1_W<1> {
        PSARC1_W::new(self)
    }
    #[doc = "Bit 13 - DOC and the MSTPCRC.MSTPC13 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarc13(&mut self) -> PSARC13_W<13> {
        PSARC13_W::new(self)
    }
    #[doc = "Bit 20 - TFU and the MSTPCRC.MSTPC20 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarc20(&mut self) -> PSARC20_W<20> {
        PSARC20_W::new(self)
    }
    #[doc = "Bit 21 - IIRFA and the MSTPCRC.MSTPC21 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarc21(&mut self) -> PSARC21_W<21> {
        PSARC21_W::new(self)
    }
    #[doc = "Bit 27 - CANFD and the MSTPCRC.MSTPC27 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarc27(&mut self) -> PSARC27_W<27> {
        PSARC27_W::new(self)
    }
    #[doc = "Bit 31 - SCE5 and the MSTPCRC.MSTPC31 bit security attribution"]
    #[inline(always)]
    #[must_use]
    pub fn psarc31(&mut self) -> PSARC31_W<31> {
        PSARC31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Security Attribution Register C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psarc](index.html) module"]
pub struct PSARC_SPEC;
impl crate::RegisterSpec for PSARC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psarc::R](R) reader structure"]
impl crate::Readable for PSARC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psarc::W](W) writer structure"]
impl crate::Writable for PSARC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSARC to value 0xffff_ffff"]
impl crate::Resettable for PSARC_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
