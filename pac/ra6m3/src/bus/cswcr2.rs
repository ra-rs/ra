#[doc = "Register `CS%sWCR2` reader"]
pub struct R(crate::R<CSWCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSWCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSWCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSWCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CS%sWCR2` writer"]
pub struct W(crate::W<CSWCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSWCR2_SPEC>;
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
impl From<crate::W<CSWCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSWCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSROFF` reader - Read-Access CS Extension Cycle Select"]
pub type CSROFF_R = crate::FieldReader<u8, CSROFF_A>;
#[doc = "Read-Access CS Extension Cycle Select\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSROFF_A {
    #[doc = "0: No wait is inserted."]
    _0X0 = 0,
}
impl From<CSROFF_A> for u8 {
    #[inline(always)]
    fn from(variant: CSROFF_A) -> Self {
        variant as _
    }
}
impl CSROFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSROFF_A> {
        match self.bits {
            0 => Some(CSROFF_A::_0X0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == CSROFF_A::_0X0
    }
}
#[doc = "Field `CSROFF` writer - Read-Access CS Extension Cycle Select"]
pub type CSROFF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSWCR2_SPEC, u8, CSROFF_A, 3, O>;
impl<'a, const O: u8> CSROFF_W<'a, O> {
    #[doc = "No wait is inserted."]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(CSROFF_A::_0X0)
    }
}
#[doc = "Field `CSWOFF` reader - Write-Access CS Extension Cycle Select"]
pub type CSWOFF_R = crate::FieldReader<u8, CSWOFF_A>;
#[doc = "Write-Access CS Extension Cycle Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSWOFF_A {
    #[doc = "0: No wait is inserted."]
    _0X0 = 0,
}
impl From<CSWOFF_A> for u8 {
    #[inline(always)]
    fn from(variant: CSWOFF_A) -> Self {
        variant as _
    }
}
impl CSWOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSWOFF_A> {
        match self.bits {
            0 => Some(CSWOFF_A::_0X0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == CSWOFF_A::_0X0
    }
}
#[doc = "Field `CSWOFF` writer - Write-Access CS Extension Cycle Select"]
pub type CSWOFF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSWCR2_SPEC, u8, CSWOFF_A, 3, O>;
impl<'a, const O: u8> CSWOFF_W<'a, O> {
    #[doc = "No wait is inserted."]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(CSWOFF_A::_0X0)
    }
}
#[doc = "Field `WDOFF` reader - Write Data Output Extension Cycle Select"]
pub type WDOFF_R = crate::FieldReader<u8, WDOFF_A>;
#[doc = "Write Data Output Extension Cycle Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDOFF_A {
    #[doc = "0: No wait is inserted."]
    _0X0 = 0,
}
impl From<WDOFF_A> for u8 {
    #[inline(always)]
    fn from(variant: WDOFF_A) -> Self {
        variant as _
    }
}
impl WDOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WDOFF_A> {
        match self.bits {
            0 => Some(WDOFF_A::_0X0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == WDOFF_A::_0X0
    }
}
#[doc = "Field `WDOFF` writer - Write Data Output Extension Cycle Select"]
pub type WDOFF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSWCR2_SPEC, u8, WDOFF_A, 3, O>;
impl<'a, const O: u8> WDOFF_W<'a, O> {
    #[doc = "No wait is inserted."]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(WDOFF_A::_0X0)
    }
}
#[doc = "Field `AWAIT` reader - Address Cycle Wait Select"]
pub type AWAIT_R = crate::FieldReader<u8, AWAIT_A>;
#[doc = "Address Cycle Wait Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AWAIT_A {
    #[doc = "0: No wait is inserted."]
    _0X0 = 0,
}
impl From<AWAIT_A> for u8 {
    #[inline(always)]
    fn from(variant: AWAIT_A) -> Self {
        variant as _
    }
}
impl AWAIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AWAIT_A> {
        match self.bits {
            0 => Some(AWAIT_A::_0X0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == AWAIT_A::_0X0
    }
}
#[doc = "Field `AWAIT` writer - Address Cycle Wait Select"]
pub type AWAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSWCR2_SPEC, u8, AWAIT_A, 2, O>;
impl<'a, const O: u8> AWAIT_W<'a, O> {
    #[doc = "No wait is inserted."]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(AWAIT_A::_0X0)
    }
}
#[doc = "Field `RDON` reader - RD Assert Wait Select"]
pub type RDON_R = crate::FieldReader<u8, RDON_A>;
#[doc = "RD Assert Wait Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RDON_A {
    #[doc = "0: No wait is inserted."]
    _0X0 = 0,
}
impl From<RDON_A> for u8 {
    #[inline(always)]
    fn from(variant: RDON_A) -> Self {
        variant as _
    }
}
impl RDON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RDON_A> {
        match self.bits {
            0 => Some(RDON_A::_0X0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == RDON_A::_0X0
    }
}
#[doc = "Field `RDON` writer - RD Assert Wait Select"]
pub type RDON_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSWCR2_SPEC, u8, RDON_A, 3, O>;
impl<'a, const O: u8> RDON_W<'a, O> {
    #[doc = "No wait is inserted."]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(RDON_A::_0X0)
    }
}
#[doc = "Field `WRON` reader - WR Assert Wait Select"]
pub type WRON_R = crate::FieldReader<u8, WRON_A>;
#[doc = "WR Assert Wait Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WRON_A {
    #[doc = "0: No wait is inserted."]
    _0X0 = 0,
}
impl From<WRON_A> for u8 {
    #[inline(always)]
    fn from(variant: WRON_A) -> Self {
        variant as _
    }
}
impl WRON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WRON_A> {
        match self.bits {
            0 => Some(WRON_A::_0X0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == WRON_A::_0X0
    }
}
#[doc = "Field `WRON` writer - WR Assert Wait Select"]
pub type WRON_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSWCR2_SPEC, u8, WRON_A, 3, O>;
impl<'a, const O: u8> WRON_W<'a, O> {
    #[doc = "No wait is inserted."]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(WRON_A::_0X0)
    }
}
#[doc = "Field `WDON` reader - Write Data Output Wait Select"]
pub type WDON_R = crate::FieldReader<u8, WDON_A>;
#[doc = "Write Data Output Wait Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDON_A {
    #[doc = "0: No wait is inserted."]
    _0X0 = 0,
}
impl From<WDON_A> for u8 {
    #[inline(always)]
    fn from(variant: WDON_A) -> Self {
        variant as _
    }
}
impl WDON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WDON_A> {
        match self.bits {
            0 => Some(WDON_A::_0X0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == WDON_A::_0X0
    }
}
#[doc = "Field `WDON` writer - Write Data Output Wait Select"]
pub type WDON_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSWCR2_SPEC, u8, WDON_A, 3, O>;
impl<'a, const O: u8> WDON_W<'a, O> {
    #[doc = "No wait is inserted."]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(WDON_A::_0X0)
    }
}
#[doc = "Field `CSON` reader - CS Assert Wait Select"]
pub type CSON_R = crate::FieldReader<u8, CSON_A>;
#[doc = "CS Assert Wait Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSON_A {
    #[doc = "0: No wait is inserted."]
    _0X0 = 0,
}
impl From<CSON_A> for u8 {
    #[inline(always)]
    fn from(variant: CSON_A) -> Self {
        variant as _
    }
}
impl CSON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSON_A> {
        match self.bits {
            0 => Some(CSON_A::_0X0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0X0`"]
    #[inline(always)]
    pub fn is_0x0(&self) -> bool {
        *self == CSON_A::_0X0
    }
}
#[doc = "Field `CSON` writer - CS Assert Wait Select"]
pub type CSON_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSWCR2_SPEC, u8, CSON_A, 3, O>;
impl<'a, const O: u8> CSON_W<'a, O> {
    #[doc = "No wait is inserted."]
    #[inline(always)]
    pub fn _0x0(self) -> &'a mut W {
        self.variant(CSON_A::_0X0)
    }
}
impl R {
    #[doc = "Bits 0:2 - Read-Access CS Extension Cycle Select"]
    #[inline(always)]
    pub fn csroff(&self) -> CSROFF_R {
        CSROFF_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Write-Access CS Extension Cycle Select"]
    #[inline(always)]
    pub fn cswoff(&self) -> CSWOFF_R {
        CSWOFF_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Write Data Output Extension Cycle Select"]
    #[inline(always)]
    pub fn wdoff(&self) -> WDOFF_R {
        WDOFF_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Address Cycle Wait Select"]
    #[inline(always)]
    pub fn await_(&self) -> AWAIT_R {
        AWAIT_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:18 - RD Assert Wait Select"]
    #[inline(always)]
    pub fn rdon(&self) -> RDON_R {
        RDON_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - WR Assert Wait Select"]
    #[inline(always)]
    pub fn wron(&self) -> WRON_R {
        WRON_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Write Data Output Wait Select"]
    #[inline(always)]
    pub fn wdon(&self) -> WDON_R {
        WDON_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - CS Assert Wait Select"]
    #[inline(always)]
    pub fn cson(&self) -> CSON_R {
        CSON_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Read-Access CS Extension Cycle Select"]
    #[inline(always)]
    #[must_use]
    pub fn csroff(&mut self) -> CSROFF_W<0> {
        CSROFF_W::new(self)
    }
    #[doc = "Bits 4:6 - Write-Access CS Extension Cycle Select"]
    #[inline(always)]
    #[must_use]
    pub fn cswoff(&mut self) -> CSWOFF_W<4> {
        CSWOFF_W::new(self)
    }
    #[doc = "Bits 8:10 - Write Data Output Extension Cycle Select"]
    #[inline(always)]
    #[must_use]
    pub fn wdoff(&mut self) -> WDOFF_W<8> {
        WDOFF_W::new(self)
    }
    #[doc = "Bits 12:13 - Address Cycle Wait Select"]
    #[inline(always)]
    #[must_use]
    pub fn await_(&mut self) -> AWAIT_W<12> {
        AWAIT_W::new(self)
    }
    #[doc = "Bits 16:18 - RD Assert Wait Select"]
    #[inline(always)]
    #[must_use]
    pub fn rdon(&mut self) -> RDON_W<16> {
        RDON_W::new(self)
    }
    #[doc = "Bits 20:22 - WR Assert Wait Select"]
    #[inline(always)]
    #[must_use]
    pub fn wron(&mut self) -> WRON_W<20> {
        WRON_W::new(self)
    }
    #[doc = "Bits 24:26 - Write Data Output Wait Select"]
    #[inline(always)]
    #[must_use]
    pub fn wdon(&mut self) -> WDON_W<24> {
        WDON_W::new(self)
    }
    #[doc = "Bits 28:30 - CS Assert Wait Select"]
    #[inline(always)]
    #[must_use]
    pub fn cson(&mut self) -> CSON_W<28> {
        CSON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CS%s Wait Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cswcr2](index.html) module"]
pub struct CSWCR2_SPEC;
impl crate::RegisterSpec for CSWCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cswcr2::R](R) reader structure"]
impl crate::Readable for CSWCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cswcr2::W](W) writer structure"]
impl crate::Writable for CSWCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CS%sWCR2 to value 0x07"]
impl crate::Resettable for CSWCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
