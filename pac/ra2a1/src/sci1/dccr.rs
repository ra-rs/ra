#[doc = "Register `DCCR` reader"]
pub struct R(crate::R<DCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCCR` writer"]
pub struct W(crate::W<DCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCCR_SPEC>;
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
impl From<crate::W<DCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCMF` reader - Data Compare Match Flag\n\nThe field is **modified** in some way after a read operation."]
pub type DCMF_R = crate::BitReader<DCMF_A>;
#[doc = "Data Compare Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMF_A {
    #[doc = "0: No matched"]
    _0 = 0,
    #[doc = "1: Matched"]
    _1 = 1,
}
impl From<DCMF_A> for bool {
    #[inline(always)]
    fn from(variant: DCMF_A) -> Self {
        variant as u8 != 0
    }
}
impl DCMF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCMF_A {
        match self.bits {
            false => DCMF_A::_0,
            true => DCMF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCMF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCMF_A::_1
    }
}
#[doc = "Field `DCMF` writer - Data Compare Match Flag"]
pub type DCMF_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, DCCR_SPEC, DCMF_A, O>;
impl<'a, const O: u8> DCMF_W<'a, O> {
    #[doc = "No matched"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCMF_A::_0)
    }
    #[doc = "Matched"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCMF_A::_1)
    }
}
#[doc = "Field `DPER` reader - Data Compare Match Parity Error Flag\n\nThe field is **modified** in some way after a read operation."]
pub type DPER_R = crate::BitReader<DPER_A>;
#[doc = "Data Compare Match Parity Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPER_A {
    #[doc = "0: No parity error occurred"]
    _0 = 0,
    #[doc = "1: A parity error has occurred"]
    _1 = 1,
}
impl From<DPER_A> for bool {
    #[inline(always)]
    fn from(variant: DPER_A) -> Self {
        variant as u8 != 0
    }
}
impl DPER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPER_A {
        match self.bits {
            false => DPER_A::_0,
            true => DPER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPER_A::_1
    }
}
#[doc = "Field `DPER` writer - Data Compare Match Parity Error Flag"]
pub type DPER_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, DCCR_SPEC, DPER_A, O>;
impl<'a, const O: u8> DPER_W<'a, O> {
    #[doc = "No parity error occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPER_A::_0)
    }
    #[doc = "A parity error has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPER_A::_1)
    }
}
#[doc = "Field `DFER` reader - Data Compare Match Framing Error Flag\n\nThe field is **modified** in some way after a read operation."]
pub type DFER_R = crate::BitReader<DFER_A>;
#[doc = "Data Compare Match Framing Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFER_A {
    #[doc = "0: No framing error occurred"]
    _0 = 0,
    #[doc = "1: A framing error has occurred"]
    _1 = 1,
}
impl From<DFER_A> for bool {
    #[inline(always)]
    fn from(variant: DFER_A) -> Self {
        variant as u8 != 0
    }
}
impl DFER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFER_A {
        match self.bits {
            false => DFER_A::_0,
            true => DFER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFER_A::_1
    }
}
#[doc = "Field `DFER` writer - Data Compare Match Framing Error Flag"]
pub type DFER_W<'a, const O: u8> = crate::BitWriter0C<'a, u8, DCCR_SPEC, DFER_A, O>;
impl<'a, const O: u8> DFER_W<'a, O> {
    #[doc = "No framing error occurred"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFER_A::_0)
    }
    #[doc = "A framing error has occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFER_A::_1)
    }
}
#[doc = "Field `IDSEL` reader - ID frame select Bit(Valid only in asynchronous mode(including multi-processor)"]
pub type IDSEL_R = crate::BitReader<IDSEL_A>;
#[doc = "ID frame select Bit(Valid only in asynchronous mode(including multi-processor)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDSEL_A {
    #[doc = "0: Always compare data regardless of the value of the MPB bit."]
    _0 = 0,
    #[doc = "1: Compare data when the MPB bit is 1 (ID frame) only."]
    _1 = 1,
}
impl From<IDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: IDSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl IDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDSEL_A {
        match self.bits {
            false => IDSEL_A::_0,
            true => IDSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDSEL_A::_1
    }
}
#[doc = "Field `IDSEL` writer - ID frame select Bit(Valid only in asynchronous mode(including multi-processor)"]
pub type IDSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, DCCR_SPEC, IDSEL_A, O>;
impl<'a, const O: u8> IDSEL_W<'a, O> {
    #[doc = "Always compare data regardless of the value of the MPB bit."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IDSEL_A::_0)
    }
    #[doc = "Compare data when the MPB bit is 1 (ID frame) only."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IDSEL_A::_1)
    }
}
#[doc = "Field `DCME` reader - Data Compare Match Enable(Valid only in asynchronous mode(including multi-processor)"]
pub type DCME_R = crate::BitReader<DCME_A>;
#[doc = "Data Compare Match Enable(Valid only in asynchronous mode(including multi-processor)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCME_A {
    #[doc = "0: Address match function is disabled."]
    _0 = 0,
    #[doc = "1: Address match function is enabled"]
    _1 = 1,
}
impl From<DCME_A> for bool {
    #[inline(always)]
    fn from(variant: DCME_A) -> Self {
        variant as u8 != 0
    }
}
impl DCME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCME_A {
        match self.bits {
            false => DCME_A::_0,
            true => DCME_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCME_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCME_A::_1
    }
}
#[doc = "Field `DCME` writer - Data Compare Match Enable(Valid only in asynchronous mode(including multi-processor)"]
pub type DCME_W<'a, const O: u8> = crate::BitWriter<'a, u8, DCCR_SPEC, DCME_A, O>;
impl<'a, const O: u8> DCME_W<'a, O> {
    #[doc = "Address match function is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCME_A::_0)
    }
    #[doc = "Address match function is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCME_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Data Compare Match Flag"]
    #[inline(always)]
    pub fn dcmf(&self) -> DCMF_R {
        DCMF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Data Compare Match Parity Error Flag"]
    #[inline(always)]
    pub fn dper(&self) -> DPER_R {
        DPER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Compare Match Framing Error Flag"]
    #[inline(always)]
    pub fn dfer(&self) -> DFER_R {
        DFER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - ID frame select Bit(Valid only in asynchronous mode(including multi-processor)"]
    #[inline(always)]
    pub fn idsel(&self) -> IDSEL_R {
        IDSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data Compare Match Enable(Valid only in asynchronous mode(including multi-processor)"]
    #[inline(always)]
    pub fn dcme(&self) -> DCME_R {
        DCME_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Compare Match Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dcmf(&mut self) -> DCMF_W<0> {
        DCMF_W::new(self)
    }
    #[doc = "Bit 3 - Data Compare Match Parity Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dper(&mut self) -> DPER_W<3> {
        DPER_W::new(self)
    }
    #[doc = "Bit 4 - Data Compare Match Framing Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dfer(&mut self) -> DFER_W<4> {
        DFER_W::new(self)
    }
    #[doc = "Bit 6 - ID frame select Bit(Valid only in asynchronous mode(including multi-processor)"]
    #[inline(always)]
    #[must_use]
    pub fn idsel(&mut self) -> IDSEL_W<6> {
        IDSEL_W::new(self)
    }
    #[doc = "Bit 7 - Data Compare Match Enable(Valid only in asynchronous mode(including multi-processor)"]
    #[inline(always)]
    #[must_use]
    pub fn dcme(&mut self) -> DCME_W<7> {
        DCME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Compare Match Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dccr](index.html) module"]
pub struct DCCR_SPEC;
impl crate::RegisterSpec for DCCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dccr::R](R) reader structure"]
impl crate::Readable for DCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dccr::W](W) writer structure"]
impl crate::Writable for DCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x19;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCCR to value 0x40"]
impl crate::Resettable for DCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
